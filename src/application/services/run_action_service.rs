use std::sync::Arc;

use crate::application::commands::ExecuteActionCommand;
use crate::application::dtos::requests::RunActionRequest;
use crate::application::dtos::responses::ExecuteActionResponse;
use crate::application::ports::inbound::RunActionPort;
use crate::application::ports::outbound::CommandBusPort;
use crate::domain::errors::StepError;

/// Application service implementing the entrypoint to run an action.
///
/// Depends only on outbound ports (`CommandBusPort`).
pub struct RunActionService {
    command_bus: Arc<dyn CommandBusPort>,
}

impl RunActionService {
    pub fn new(command_bus: Arc<dyn CommandBusPort>) -> Self {
        Self { command_bus }
    }
}

impl RunActionPort for RunActionService {
    fn execute(&self, request: RunActionRequest) -> Result<ExecuteActionResponse, StepError> {
        let cmd = ExecuteActionCommand::new(
            request.action_ref().to_string().to_string(),
            request.step().clone().clone(),
            request.repo_path().to_path_buf().to_path_buf(),
            request.env().clone().clone(),
            request.context().clone().clone(),
            request.container().clone().clone(),
        );
        self.command_bus.dispatch_action(cmd)
    }
}
