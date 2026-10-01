use crate::{
    domain::{errors::StepError, messages::commands::ExecuteActionCommand},
    dtos::responses::ExecuteActionResponse,
    ports::outbound::container_port::ContainerPort,
};

/// Publishes an action command to the bound infrastructure command transport.
pub trait ActionCommandPublisherPort: Send + Sync {
    fn publish(
        &self,
        command: ExecuteActionCommand<dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError>;
}
