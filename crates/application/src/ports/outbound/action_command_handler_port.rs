use std::sync::Arc;

use crate::{
    domain::{errors::StepError, messages::commands::ExecuteActionPayload},
    dtos::responses::ExecuteActionResponse,
    ports::outbound::container_port::ContainerPort,
};

/// Handles action commands against a supplied container.
pub trait ActionCommandHandlerPort: Send + Sync {
    /// Executes the action described by `command` in `container` and returns
    /// its exit status and captured output.
    ///
    /// # Errors
    ///
    /// Returns [`StepError`] when the action request cannot be encoded or
    /// execution fails.
    fn handle(
        &self,
        command: ExecuteActionPayload,
        container: Arc<dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError>;
}
