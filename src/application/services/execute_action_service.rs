use std::{collections::HashMap, path::PathBuf, sync::Arc};

use crate::application::dtos::requests::ExecuteActionRequest;
use crate::application::dtos::requests::LoadActionDefinitionRequest;
use crate::application::dtos::requests::ResolveActionDirectoryRequest;
use crate::application::dtos::requests::ResolveActionInputsRequest;
use crate::application::dtos::requests::RunCompositeActionRequest;
use crate::application::dtos::requests::RunNodeActionRequest;
use crate::application::dtos::responses::ExecuteActionResponse;
use crate::application::dtos::responses::ResolvedActionDirectoryResponse;
use crate::application::errors::ExecuteActionError;
use crate::application::ports::outbound::container_port::ContainerPort;
use crate::application::ports::{
    inbound::execute_action_port::ExecuteActionPort,
    outbound::{
        load_action_definition_port::LoadActionDefinitionPort,
        resolve_action_directory_port::ResolveActionDirectoryPort,
        resolve_action_inputs_port::ResolveActionInputsPort,
        run_composite_action_port::RunCompositeActionPort, run_node_action_port::RunNodeActionPort,
    },
};
use crate::domain::errors::ActionError;
use crate::domain::errors::StepError;
use crate::domain::services::step_factory::StepFactory;
use crate::domain::value_objects::ActionDefinition;
use crate::domain::value_objects::ActionRuntime;

/// Application service that runs the action a step references.
///
/// Resolves where the action lives, loads its definition and inputs, and
/// dispatches on how the action declares it runs: composite actions run their
/// steps in the job's container, JavaScript actions are copied in and run with
/// node, and container actions are reported as unsupported instead of being
/// silently skipped.
pub struct ExecuteActionService<'container> {
    container: &'container dyn ContainerPort,
    directory_resolver: Arc<dyn ResolveActionDirectoryPort>,
    definition_loader: Arc<dyn LoadActionDefinitionPort>,
    input_resolver: Arc<dyn ResolveActionInputsPort>,
    composite_runner: Arc<dyn RunCompositeActionPort>,
    node_runner: Arc<dyn RunNodeActionPort>,
}

enum ActionDirectoryResolution {
    Skipped(ExecuteActionResponse),
    Directory(PathBuf),
}

impl<'container> ExecuteActionService<'container> {
    pub fn new(
        container: &'container dyn ContainerPort,
        directory_resolver: Arc<dyn ResolveActionDirectoryPort>,
        definition_loader: Arc<dyn LoadActionDefinitionPort>,
        input_resolver: Arc<dyn ResolveActionInputsPort>,
        composite_runner: Arc<dyn RunCompositeActionPort>,
        node_runner: Arc<dyn RunNodeActionPort>,
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
            .execute(ResolveActionDirectoryRequest::new(
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
            .execute(LoadActionDefinitionRequest::new(action_dir.to_path_buf()))
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
            .execute(ResolveActionInputsRequest::new(definition, &step))
    }

    fn execute_loaded_action(
        &self,
        request: &ExecuteActionRequest,
        definition: &ActionDefinition,
        inputs: &HashMap<String, String>,
        action_dir: &std::path::Path,
    ) -> Result<ExecuteActionResponse, StepError> {
        match definition.runs() {
            ActionRuntime::Composite { steps } => self.composite_runner.execute(
                RunCompositeActionRequest::new(steps.as_slice(), inputs, action_dir, request),
                self.container,
            ),
            ActionRuntime::Node12 { main }
            | ActionRuntime::Node16 { main }
            | ActionRuntime::Node20 { main } => self
                .node_runner
                .execute(
                    RunNodeActionRequest::new(
                        action_dir,
                        main.as_str(),
                        inputs.clone(),
                        request.env().clone(),
                    ),
                    self.container,
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

impl ExecuteActionPort for ExecuteActionService<'_> {
    fn execute(
        &self,
        request: ExecuteActionRequest,
    ) -> Result<ExecuteActionResponse, ExecuteActionError> {
        self.run_action(&request).map_err(ExecuteActionError::Step)
    }
}
