use std::sync::Arc;

use crate::{
    domain::{errors::StepError, messages::commands::ExecuteActionPayload},
    dtos::responses::ExecuteActionResponse,
    ports::outbound::container_port::ContainerPort,
};

/// Publishes action commands for execution.
pub trait ActionCommandPublisherPort: Send + Sync {
    /// Routes `command` and its container to an action handler and returns the
    /// resulting action status and output.
    ///
    /// # Errors
    ///
    /// Returns [`StepError`] when dispatch or action execution fails.
    fn publish(
        &self,
        command: ExecuteActionPayload,
        container: Arc<dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError>;
}
