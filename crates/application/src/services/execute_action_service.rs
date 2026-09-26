use std::{collections::HashMap, path::PathBuf, sync::Arc};

use crate::{
    domain::{
        errors::{ActionError, StepError},
        services::step_factory::StepFactory,
        value_objects::{ActionDefinition, ActionRuntime},
    },
    dtos::{
        requests::{
            ExecuteActionRequest, LoadActionDefinitionRequest, ResolveActionDirectoryRequest,
            ResolveActionInputsRequest, RunCompositeActionRequest, RunNodeActionRequest,
        },
        responses::{ExecuteActionResponse, ResolvedActionDirectoryResponse},
    },
    errors::ExecuteActionError,
    ports::{
        inbound::execute_action_port::ExecuteActionPort,
        outbound::{
            action_definition_loader_port::ActionDefinitionLoaderPort,
            action_directory_resolver_port::ActionDirectoryResolverPort,
            action_inputs_resolver_port::ActionInputsResolverPort,
            composite_action_runner_port::CompositeActionRunnerPort, container_port::ContainerPort,
            node_action_runner_port::NodeActionRunnerPort,
        },
    },
};

/// Application service that runs the action a step references.
///
/// Resolves where the action lives, loads its definition and inputs, and
/// dispatches on how the action declares it runs: composite actions run their
/// steps in the job's container, JavaScript actions are copied in and run with
/// node, and container actions are reported as unsupported instead of being
/// silently skipped.
pub struct ExecuteActionService {
    container: Arc<dyn ContainerPort>,
    directory_resolver: Arc<dyn ActionDirectoryResolverPort>,
    definition_loader: Arc<dyn ActionDefinitionLoaderPort>,
    input_resolver: Arc<dyn ActionInputsResolverPort>,
    composite_runner: Arc<dyn CompositeActionRunnerPort>,
    node_runner: Arc<dyn NodeActionRunnerPort>,
}

enum ActionDirectoryResolution {
    Skipped(ExecuteActionResponse),
    Directory(PathBuf),
}

impl ExecuteActionService {
    pub fn new(
        container: Arc<dyn ContainerPort>,
        directory_resolver: Arc<dyn ActionDirectoryResolverPort>,
        definition_loader: Arc<dyn ActionDefinitionLoaderPort>,
        input_resolver: Arc<dyn ActionInputsResolverPort>,
        composite_runner: Arc<dyn CompositeActionRunnerPort>,
        node_runner: Arc<dyn NodeActionRunnerPort>,
    ) -> Self {
        Self {
            container,
            directory_resolver,
            definition_loader,
            input_resolver,
            composite_runner,
            node_runner,
        }
    }

    fn run_action(
        &self,
        request: &ExecuteActionRequest,
    ) -> Result<ExecuteActionResponse, StepError> {
        let action_dir = match self.resolve_action_directory(request)? {
            ActionDirectoryResolution::Skipped(response) => return Ok(response),
            ActionDirectoryResolution::Directory(directory) => directory,
        };
        let definition = self.load_action_definition(request, &action_dir)?;
        let inputs = self.resolve_action_inputs(&definition, request)?;

        self.execute_loaded_action(request, &definition, &inputs, &action_dir)
    }

    fn resolve_action_directory(
        &self,
        request: &ExecuteActionRequest,
    ) -> Result<ActionDirectoryResolution, StepError> {
        match self
            .directory_resolver
            .resolve(ResolveActionDirectoryRequest::new(
                request.action_ref().to_string(),
                request.repo_path().to_path_buf(),
            ))? {
            ResolvedActionDirectoryResponse::Skipped(response) => {
                Ok(ActionDirectoryResolution::Skipped(response))
            }
            ResolvedActionDirectoryResponse::Directory(directory) => {
                Ok(ActionDirectoryResolution::Directory(directory))
            }
        }
    }

    fn load_action_definition(
        &self,
        request: &ExecuteActionRequest,
        action_dir: &std::path::Path,
    ) -> Result<ActionDefinition, StepError> {
        self.definition_loader
            .load(LoadActionDefinitionRequest::new(action_dir.to_path_buf()))
            .map_err(|error| {
                StepError::new(format!(
                    "failed to load action '{}': {}",
                    request.action_ref(),
                    error.message()
                ))
            })
    }

    fn resolve_action_inputs(
        &self,
        definition: &ActionDefinition,
        request: &ExecuteActionRequest,
    ) -> Result<HashMap<String, String>, StepError> {
        let step = StepFactory::from_text(request.step())?;
        self.input_resolver
            .resolve(ResolveActionInputsRequest::new(definition.clone(), step))
    }

    fn execute_loaded_action(
        &self,
        request: &ExecuteActionRequest,
        definition: &ActionDefinition,
        inputs: &HashMap<String, String>,
        action_dir: &std::path::Path,
    ) -> Result<ExecuteActionResponse, StepError> {
        match definition.runs() {
            ActionRuntime::Composite { steps } => self.composite_runner.run(
                RunCompositeActionRequest::new(steps.as_slice(), inputs, action_dir, request),
                self.container.clone(),
            ),
            ActionRuntime::Node12 { main }
            | ActionRuntime::Node16 { main }
            | ActionRuntime::Node20 { main } => self
                .node_runner
                .run(
                    RunNodeActionRequest::new(
                        action_dir,
                        main.as_str(),
                        inputs.clone(),
                        request.env().clone(),
                    ),
                    self.container.clone(),
                )
                .map(|result| {
                    ExecuteActionResponse::new(result.exit_code(), result.stdout(), result.stderr())
                }),
            ActionRuntime::Docker { image } => Err(StepError::new(
                ActionError::Unsupported(format!(
                    "action '{}' runs the container image '{image}', which cannot be executed yet",
                    request.action_ref()
                ))
                .to_string(),
            )),
        }
    }
}

impl ExecuteActionPort for ExecuteActionService {
    fn execute(
        &self,
        request: ExecuteActionRequest,
    ) -> Result<ExecuteActionResponse, ExecuteActionError> {
        self.run_action(&request).map_err(ExecuteActionError::Step)
    }
}
