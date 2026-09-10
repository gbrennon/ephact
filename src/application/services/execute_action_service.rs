use std::{collections::HashMap, path::PathBuf};

use crate::application::ports::{
    inbound::execute_action_port::ExecuteActionPort,
    outbound::{
        load_action_definition_port::LoadActionDefinitionPort,
        resolve_action_directory_port::ResolveActionDirectoryPort,
        resolve_action_inputs_port::ResolveActionInputsPort,
        run_composite_action_port::RunCompositeActionPort, run_node_action_port::RunNodeActionPort,
    },
};
use crate::{
    application::dtos::{
        ExecuteActionRequest, ExecuteActionResponse, LoadActionDefinitionRequest,
        ResolveActionDirectoryRequest, ResolveActionInputsRequest, ResolvedActionDirectory,
        RunCompositeActionRequest, RunNodeActionRequest,
    },
    domain::{
        errors::{ActionError, StepError},
        value_objects::{ActionDefinition, ActionRuntime},
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
    directory_resolver: Box<dyn ResolveActionDirectoryPort>,
    definition_loader: Box<dyn LoadActionDefinitionPort>,
    input_resolver: Box<dyn ResolveActionInputsPort>,
    composite_runner: Box<dyn RunCompositeActionPort>,
    node_runner: Box<dyn RunNodeActionPort>,
}

enum ActionDirectoryResolution {
    Skipped(ExecuteActionResponse),
    Directory(PathBuf),
}

impl ExecuteActionService {
    pub fn new(
        directory_resolver: Box<dyn ResolveActionDirectoryPort>,
        definition_loader: Box<dyn LoadActionDefinitionPort>,
        input_resolver: Box<dyn ResolveActionInputsPort>,
        composite_runner: Box<dyn RunCompositeActionPort>,
        node_runner: Box<dyn RunNodeActionPort>,
    ) -> Self {
        Self {
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
                request.action_ref(),
                request.repo_path(),
            ))? {
            ResolvedActionDirectory::Skipped(response) => {
                Ok(ActionDirectoryResolution::Skipped(response))
            }
            ResolvedActionDirectory::Directory(directory) => {
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
            .execute(LoadActionDefinitionRequest::new(action_dir))
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
        self.input_resolver
            .execute(ResolveActionInputsRequest::new(definition, request.step()))
    }

    fn execute_loaded_action(
        &self,
        request: &ExecuteActionRequest,
        definition: &ActionDefinition,
        inputs: &HashMap<String, String>,
        action_dir: &std::path::Path,
    ) -> Result<ExecuteActionResponse, StepError> {
        match definition.runs() {
            ActionRuntime::Composite { steps } => {
                self.composite_runner
                    .execute(RunCompositeActionRequest::new(
                        steps, inputs, action_dir, request,
                    ))
            }
            ActionRuntime::Node12 { main }
            | ActionRuntime::Node16 { main }
            | ActionRuntime::Node20 { main } => self
                .node_runner
                .execute(RunNodeActionRequest::new(
                    action_dir,
                    main,
                    inputs,
                    request.env(),
                    request.container().as_ref(),
                ))
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
    fn execute(&self, request: ExecuteActionRequest) -> Result<ExecuteActionResponse, StepError> {
        self.run_action(&request)
    }
}
