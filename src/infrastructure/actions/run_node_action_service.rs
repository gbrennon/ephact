use std::sync::Arc;

use crate::{
    application::{
        dtos::{
            requests::{
                BuildActionInputEnvironmentRequest, CopyActionToContainerRequest,
                ResolveNodeBinaryRequest, RunNodeActionRequest,
            },
            responses::RunNodeActionResponse,
        },
        ports::outbound::{
            container_port::ContainerPort, node_action_runner_port::NodeActionRunnerPort,
        },
    },
    domain::{errors::StepError, value_objects::ShellCommand},
    infrastructure::{
        actions::{
            BuildActionInputEnvironmentPort,
            copy_action_to_container_port::CopyActionToContainerPort,
            resolve_node_binary_port::ResolveNodeBinaryPort,
        },
        containers::workspace::CONTAINER_WORKSPACE,
    },
};

/// Service that runs a JavaScript action: copies it into the container,
/// exposes its inputs as environment variables, and runs its entry point.
pub struct RunNodeActionService {
    action_copier: Box<dyn CopyActionToContainerPort>,
    environment_builder: Box<dyn BuildActionInputEnvironmentPort>,
    node_binary_resolver: Box<dyn ResolveNodeBinaryPort>,
}

impl RunNodeActionService {
    pub fn new(
        action_copier: Box<dyn CopyActionToContainerPort>,
        environment_builder: Box<dyn BuildActionInputEnvironmentPort>,
        node_binary_resolver: Box<dyn ResolveNodeBinaryPort>,
    ) -> Self {
        Self {
            action_copier,
            environment_builder,
            node_binary_resolver,
        }
    }
}

impl NodeActionRunnerPort for RunNodeActionService {
    fn run(
        &self,
        request: RunNodeActionRequest,
        container: Arc<dyn ContainerPort>,
    ) -> Result<RunNodeActionResponse, StepError> {
        let container_dir = self
            .action_copier
            .execute(CopyActionToContainerRequest::new(
                request.action_dir().to_path_buf(),
                container.clone(),
            ))?;

        let action_request = BuildActionInputEnvironmentRequest::new(
            request.env().clone(),
            request.inputs().clone(),
            container_dir.clone(),
        );
        let action_response = self.environment_builder.execute(action_request);
        let binary = self
            .node_binary_resolver
            .execute(ResolveNodeBinaryRequest::new(container.clone()));

        let entry_point = request.entry_point();
        let command = ShellCommand::new(
            vec![binary, format!("{container_dir}/{entry_point}")],
            Some(CONTAINER_WORKSPACE.into()),
            action_response.into_env(),
        );

        container
            .clone()
            .exec(command.argv(), command.working_directory(), command.env())
            .map(|result| {
                RunNodeActionResponse::new(
                    result.exit_code(),
                    result.stdout().to_string(),
                    result.stderr().to_string(),
                )
            })
            .map_err(|error| StepError::new(format!("failed to run node action: {error:?}")))
    }
}
