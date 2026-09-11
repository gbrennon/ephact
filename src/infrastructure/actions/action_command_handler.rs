use crate::application::dtos::requests::ExecuteActionRequest;
use crate::application::dtos::responses::ExecuteActionResponse;
use crate::application::ports::inbound::execute_action_port::ExecuteActionPort;
use crate::application::ports::outbound::container_port::ContainerPort;
use crate::domain::errors::StepError;
use crate::domain::messages::commands::ExecuteActionCommand;

/// Infrastructure command handler that processes `ExecuteActionCommand`.
pub struct ActionCommandHandler {
    executor: Box<dyn ExecuteActionPort>,
}

impl ActionCommandHandler {
    pub fn new(executor: Box<dyn ExecuteActionPort>) -> Self {
        Self { executor }
    }

    pub fn handle<'a>(
        &self,
        cmd: ExecuteActionCommand<'a, dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError> {
        let (action_ref, step, repo_path, env, context, container) = cmd.into_parts();
        let req = ExecuteActionRequest::new(action_ref, step, repo_path, env, context, container);
        self.executor.execute(req)
    }
}
