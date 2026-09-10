use crate::{
    application::{
        dtos::{ExecuteActionCommand, ExecuteActionRequest, ExecuteActionResponse},
        ports::inbound::execute_action_port::ExecuteActionPort,
    },
    domain::errors::StepError,
};

/// Infrastructure command handler that processes `ExecuteActionCommand`.
pub struct ActionCommandHandler {
    executor: Box<dyn ExecuteActionPort>,
}

impl ActionCommandHandler {
    pub fn new(executor: Box<dyn ExecuteActionPort>) -> Self {
        Self { executor }
    }

    pub fn handle(&self, cmd: ExecuteActionCommand) -> Result<ExecuteActionResponse, StepError> {
        let (action_ref, step, repo_path, env, context, container) = cmd.into_parts();
        let req = ExecuteActionRequest::new(action_ref, step, repo_path, env, context, container);
        self.executor.execute(req)
    }
}
