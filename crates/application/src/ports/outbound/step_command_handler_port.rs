use crate::{
    domain::{errors::StepError, messages::commands::ExecuteStepCommand},
    dtos::responses::ExecutedStepResponse,
    ports::outbound::container_port::ContainerPort,
};

/// Handles a step command routed by the infrastructure command transport.
pub trait StepCommandHandlerPort: Send + Sync {
    fn handle(
        &self,
        command: ExecuteStepCommand<dyn ContainerPort>,
    ) -> Result<ExecutedStepResponse, StepError>;
}
