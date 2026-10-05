use std::sync::Arc;

use crate::{
    domain::{errors::StepError, messages::commands::ExecuteStepPayload},
    dtos::responses::ExecutedStepResponse,
    ports::outbound::container_port::ContainerPort,
};

/// Handles step execution commands against a container.
pub trait StepCommandHandlerPort: Send + Sync {
    /// Executes `command` in `container` and returns the resolved step and its
    /// execution output.
    ///
    /// # Errors
    ///
    /// Returns [`StepError`] when the command cannot be encoded or executed.
    fn handle(
        &self,
        command: ExecuteStepPayload,
        container: Arc<dyn ContainerPort>,
    ) -> Result<ExecutedStepResponse, StepError>;
}
