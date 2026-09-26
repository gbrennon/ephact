use std::sync::Arc;

use crate::{
    domain::{
        errors::StepError, messages::commands::ExecuteActionCommand,
        services::evaluation_context_mapper::EvaluationContextMapper,
    },
    dtos::{requests::RunActionRequest, responses::ExecuteActionResponse},
    errors::RunActionError,
    ports::{
        inbound::RunActionPort,
        outbound::{
            action_command_bus_port::ActionCommandBusPort, container_port::ContainerPort,
            step_text_codec_port::StepTextCodecPort,
        },
    },
};

pub struct RunActionService {
    command_bus: Box<dyn ActionCommandBusPort>,
    container: Arc<dyn ContainerPort>,
    step_codec: Arc<dyn StepTextCodecPort>,
}

impl RunActionService {
    pub fn new(
        command_bus: Box<dyn ActionCommandBusPort>,
        container: Arc<dyn ContainerPort>,
        step_codec: Arc<dyn StepTextCodecPort>,
    ) -> Self {
        Self {
            command_bus,
            container,
            step_codec,
        }
    }
}

impl RunActionPort for RunActionService {
    fn execute(&self, request: RunActionRequest) -> Result<ExecuteActionResponse, RunActionError> {
        let step = self
            .step_codec
            .decode(request.step())
            .map_err(RunActionError::Step)?;
        let context = EvaluationContextMapper::from_parts(request.context().to_vec())
            .map_err(|error| RunActionError::Step(StepError::new(error.to_string())))?;
        let cmd = ExecuteActionCommand::new(
            request.action_ref().to_string(),
            step,
            request.repo_path().to_path_buf(),
            request.env().clone(),
            self.container.clone(),
        )
        .with_context(context);
        self.command_bus.dispatch(cmd).map_err(RunActionError::Step)
    }
}
