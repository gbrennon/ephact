use crate::application::dtos::requests::BuildActionInputEnvironmentRequest;
use crate::application::dtos::requests::CopyActionToContainerRequest;
use crate::application::dtos::requests::ResolveNodeBinaryRequest;
use crate::application::dtos::requests::RunNodeActionRequest;
use crate::application::dtos::responses::RunNodeActionResponse;
use crate::application::ports::outbound::run_node_action_port::RunNodeActionPort;
use crate::domain::errors::StepError;
use crate::domain::value_objects::ShellCommand;
use crate::infrastructure::actions::BuildActionInputEnvironmentPort;
use crate::infrastructure::actions::copy_action_to_container_port::CopyActionToContainerPort;
use crate::infrastructure::actions::resolve_node_binary_port::ResolveNodeBinaryPort;
use crate::infrastructure::containers::workspace::CONTAINER_WORKSPACE;

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

impl RunNodeActionPort for RunNodeActionService {
    fn execute(
        &self,
        request: RunNodeActionRequest<'_>,
    ) -> Result<RunNodeActionResponse, StepError> {
        let container_dir = self
            .action_copier
            .execute(CopyActionToContainerRequest::new(
                request.action_dir(),
                request.container(),
            ))?;

        let action_request = BuildActionInputEnvironmentRequest::new(
            request.env(),
            request.inputs(),
            &container_dir,
        );
        let action_response = self.environment_builder.execute(action_request);
        let binary = self
            .node_binary_resolver
            .execute(ResolveNodeBinaryRequest::new(request.container()));

        let entry_point = request.entry_point();
        let command = ShellCommand::new(
            vec![binary, format!("{container_dir}/{entry_point}")],
            Some(CONTAINER_WORKSPACE.into()),
            action_response.into_env(),
        );

        request
            .container()
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
