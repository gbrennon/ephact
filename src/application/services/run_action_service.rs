use std::sync::Arc;

use crate::application::dtos::requests::RunActionRequest;
use crate::application::dtos::responses::ExecuteActionResponse;
use crate::application::errors::RunActionError;
use crate::application::ports::inbound::RunActionPort;
use crate::application::ports::outbound::action_command_bus_port::ActionCommandBusPort;
use crate::application::ports::outbound::container_port::ContainerPort;
use crate::domain::errors::StepError;
use crate::domain::messages::commands::ExecuteActionCommand;
use crate::domain::services::evaluation_context_mapper::EvaluationContextMapper;
use crate::domain::services::step_factory::StepFactory;

pub struct RunActionService {
    command_bus: Box<dyn ActionCommandBusPort>,
    container: Arc<dyn ContainerPort>,
}

impl RunActionService {
    pub fn new(
        command_bus: Box<dyn ActionCommandBusPort>,
        container: Arc<dyn ContainerPort>,
    ) -> Self {
        Self {
            command_bus,
            container,
        }
    }
}

impl RunActionPort for RunActionService {
    fn execute(&self, request: RunActionRequest) -> Result<ExecuteActionResponse, RunActionError> {
        let step = StepFactory::from_text(request.step()).map_err(RunActionError::Step)?;
        let context = EvaluationContextMapper::from_parts(request.context().to_vec())
            .map_err(|error| RunActionError::Step(StepError::new(error.to_string())))?;
        let cmd = ExecuteActionCommand::new(
            request.action_ref().to_string(),
            step,
            request.repo_path().to_path_buf(),
            request.env().clone(),
            self.container.as_ref(),
        )
        .with_context(context);
        self.command_bus.dispatch(cmd).map_err(RunActionError::Step)
    }
}
