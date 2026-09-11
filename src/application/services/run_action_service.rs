use crate::application::dtos::requests::RunActionRequest;
use crate::application::dtos::responses::ExecuteActionResponse;
use crate::application::ports::inbound::RunActionPort;
use crate::application::ports::outbound::command_bus_port::ActionCommandBusPort;
use crate::domain::errors::StepError;
use crate::domain::messages::commands::ExecuteActionCommand;

/// Application service implementing the entrypoint to run an action.
///
/// Depends only on outbound ports (`ActionCommandBusPort`).
pub struct RunActionService {
    command_bus: Box<ActionCommandBusPort>,
}

impl RunActionService {
    pub fn new(command_bus: Box<ActionCommandBusPort>) -> Self {
        Self { command_bus }
    }
}

impl RunActionPort for RunActionService {
    fn execute(&self, request: RunActionRequest<'_>) -> Result<ExecuteActionResponse, StepError> {
        let cmd = ExecuteActionCommand::new(
            request.action_ref().to_string(),
            request.step().clone(),
            request.repo_path().to_path_buf(),
            request.env().clone(),
            request.context().clone(),
            request.container(),
        );
        self.command_bus.dispatch(cmd)
    }
}
