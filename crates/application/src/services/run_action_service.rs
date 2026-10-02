use std::sync::Arc;

use crate::{
    domain::messages::commands::ExecuteActionCommand,
    dtos::{requests::RunActionRequest, responses::ExecuteActionResponse},
    errors::RunActionError,
    ports::{
        inbound::RunActionPort,
        outbound::{
            Command, CommandPublisherPort, CommandResponse, container_port::ContainerPort,
            step_text_codec_port::StepTextCodecPort,
        },
    },
};

pub struct RunActionService {
    command_bus: Box<dyn CommandPublisherPort>,
    container: Arc<dyn ContainerPort>,
    step_codec: Arc<dyn StepTextCodecPort>,
}

impl RunActionService {
    pub fn new(
        command_bus: Box<dyn CommandPublisherPort>,
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
        let context = request.context().clone();
        let cmd = ExecuteActionCommand::new(
            request.action_ref().to_string(),
            step,
            request.repo_path().to_path_buf(),
            request.env().clone(),
            self.container.clone(),
        )
        .with_context(context);
        self.command_bus
            .publish(Command::Action(cmd))
            .map_err(|error| match error {
                crate::ports::outbound::CommandError::Step(error) => RunActionError::Step(error),
                error => {
                    RunActionError::Step(crate::domain::errors::StepError::new(error.to_string()))
                }
            })
            .and_then(|response| match response {
                CommandResponse::Action(response) => Ok(response),
                _ => Err(RunActionError::Step(crate::domain::errors::StepError::new(
                    "unexpected command response for action",
                ))),
            })
    }
}
