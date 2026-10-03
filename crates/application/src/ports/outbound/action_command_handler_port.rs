use std::sync::Arc;

use crate::{
    domain::{errors::StepError, messages::commands::ExecuteActionPayload},
    dtos::responses::ExecuteActionResponse,
    ports::outbound::container_port::ContainerPort,
};

pub trait ActionCommandHandlerPort: Send + Sync {
    fn handle(
        &self,
        command: ExecuteActionPayload,
        container: Arc<dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError>;
}
