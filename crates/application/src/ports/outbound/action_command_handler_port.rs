use crate::{
    domain::{errors::StepError, messages::commands::ExecuteActionCommand},
    dtos::responses::ExecuteActionResponse,
    ports::outbound::container_port::ContainerPort,
};

pub trait ActionCommandHandlerPort: Send + Sync {
    fn handle(
        &self,
        command: ExecuteActionCommand<dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError>;
}
