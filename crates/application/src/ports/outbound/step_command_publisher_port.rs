use std::sync::Arc;

use crate::{
    domain::{errors::StepError, messages::commands::ExecuteStepPayload},
    dtos::responses::ExecutedStepResponse,
    ports::outbound::container_port::ContainerPort,
};

pub trait StepCommandPublisherPort: Send + Sync {
    fn publish(
        &self,
        command: ExecuteStepPayload,
        container: Arc<dyn ContainerPort>,
    ) -> Result<ExecutedStepResponse, StepError>;
}
