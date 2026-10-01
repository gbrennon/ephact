use crate::{
    domain::{errors::StepError, messages::commands::ExecuteActionCommand},
    dtos::responses::ExecuteActionResponse,
    ports::outbound::container_port::ContainerPort,
};

/// Handles an action command routed by the infrastructure command transport.
pub trait ActionCommandHandlerPort: Send + Sync {
    fn handle(
        &self,
        command: ExecuteActionCommand<dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError>;
}
