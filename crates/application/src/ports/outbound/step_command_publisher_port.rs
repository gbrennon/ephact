use std::sync::Arc;

use crate::{
    domain::{errors::StepError, messages::commands::ExecuteStepPayload},
    dtos::responses::ExecutedStepResponse,
    ports::outbound::container_port::ContainerPort,
};

/// Publishes step execution commands.
pub trait StepCommandPublisherPort: Send + Sync {
    /// Routes `command` and its container to a step handler and returns the
    /// execution response.
    ///
    /// # Errors
    ///
    /// Returns [`StepError`] when dispatch or step execution fails.
    fn publish(
        &self,
        command: ExecuteStepPayload,
        container: Arc<dyn ContainerPort>,
    ) -> Result<ExecutedStepResponse, StepError>;
}
