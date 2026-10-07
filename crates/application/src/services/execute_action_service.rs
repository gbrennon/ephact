use std::{collections::HashMap, path::PathBuf, sync::Arc};

use crate::{
    domain::{
        errors::{ActionError, StepError},
        value_objects::{ActionDefinition, ActionExecutionContext, ActionRuntime},
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
            node_action_runner_port::NodeActionRunnerPort, step_text_codec_port::StepTextCodecPort,
        },
    },
};

pub type ExecuteActionResolutionDependencies = (
    Arc<dyn ActionDirectoryResolverPort>,
    Arc<dyn ActionDefinitionLoaderPort>,
    Arc<dyn ActionInputsResolverPort>,
);

pub type ExecuteActionExecutionDependencies = (
    Arc<dyn CompositeActionRunnerPort>,
    Arc<dyn NodeActionRunnerPort>,
    Arc<dyn StepTextCodecPort>,
);

#[derive(Clone)]
pub struct ExecuteActionDependencies {
    directory_resolver: Arc<dyn ActionDirectoryResolverPort>,
    definition_loader: Arc<dyn ActionDefinitionLoaderPort>,
    input_resolver: Arc<dyn ActionInputsResolverPort>,
    composite_runner: Arc<dyn CompositeActionRunnerPort>,
    node_runner: Arc<dyn NodeActionRunnerPort>,
    step_codec: Arc<dyn StepTextCodecPort>,
}

impl ExecuteActionDependencies {
    pub fn new(
        resolution_dependencies: ExecuteActionResolutionDependencies,
        execution_dependencies: ExecuteActionExecutionDependencies,
    ) -> Self {
        let (directory_resolver, definition_loader, input_resolver) = resolution_dependencies;
        let (composite_runner, node_runner, step_codec) = execution_dependencies;
        Self {
            directory_resolver,
            definition_loader,
            input_resolver,
            composite_runner,
            node_runner,
            step_codec,
        }
    }
}

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
    step_codec: Arc<dyn StepTextCodecPort>,
}

enum ActionDirectoryResolution {
    Skipped(ExecuteActionResponse),
    Directory(PathBuf),
}

impl ExecuteActionService {
    pub fn new(container: Arc<dyn ContainerPort>, dependencies: ExecuteActionDependencies) -> Self {
        Self {
            container,
            directory_resolver: dependencies.directory_resolver,
            definition_loader: dependencies.definition_loader,
            input_resolver: dependencies.input_resolver,
            composite_runner: dependencies.composite_runner,
            node_runner: dependencies.node_runner,
            step_codec: dependencies.step_codec,
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

        let context = ActionExecutionContext::new(action_dir, definition, inputs);
        self.execute_loaded_action(request, &context)
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
        let step = self.step_codec.decode(request.step())?;
        self.input_resolver
            .resolve(ResolveActionInputsRequest::new(definition.clone(), step))
    }

    fn execute_loaded_action(
        &self,
        request: &ExecuteActionRequest,
        context: &ActionExecutionContext,
    ) -> Result<ExecuteActionResponse, StepError> {
        match context.definition().runs() {
            ActionRuntime::Composite { steps } => self.composite_runner.run(
                RunCompositeActionRequest::new(
                    steps.as_slice(),
                    context.inputs(),
                    context.action_directory(),
                    request,
                ),
                self.container.clone(),
            ),
            ActionRuntime::Node12 { main }
            | ActionRuntime::Node16 { main }
            | ActionRuntime::Node20 { main }
            | ActionRuntime::Node24 { main } => self
                .node_runner
                .run(
                    RunNodeActionRequest::new(
                        context.action_directory(),
                        main.as_str(),
                        context.inputs().clone(),
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
