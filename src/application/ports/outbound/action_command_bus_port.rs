use crate::{
    application::{
        dtos::responses::ExecuteActionResponse, ports::outbound::container_port::ContainerPort,
    },
    domain::{errors::StepError, messages::commands::ExecuteActionCommand},
};

/// Dispatches an action execution command and returns its outcome.
pub trait ActionCommandBusPort: Send + Sync {
    /// Dispatches an action command and returns its execution outcome.
    ///
    /// # Errors
    ///
    /// Returns [`StepError`] when dispatching or action execution fails.
    fn dispatch(
        &self,
        command: ExecuteActionCommand<dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError>;
}
