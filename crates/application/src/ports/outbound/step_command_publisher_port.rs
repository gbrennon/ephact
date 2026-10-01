use crate::{
    domain::{errors::StepError, messages::commands::ExecuteStepCommand},
    dtos::responses::ExecutedStepResponse,
    ports::outbound::container_port::ContainerPort,
};

/// Publishes a step command to the bound infrastructure command transport.
pub trait StepCommandPublisherPort: Send + Sync {
    fn publish(
        &self,
        command: ExecuteStepCommand<dyn ContainerPort>,
    ) -> Result<ExecutedStepResponse, StepError>;
}
