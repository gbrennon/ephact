use crate::{
    application::{
        dtos::responses::ExecuteActionResponse, ports::outbound::container_port::ContainerPort,
    },
    domain::{errors::StepError, messages::commands::ExecuteActionCommand},
};

/// Outbound port for dispatching an action execution command.
///
/// Implementations route an [`ExecuteActionCommand`] to whatever produces its
/// outcome and return the resulting [`ExecuteActionResponse`].
pub trait ActionCommandBusPort: Send + Sync {
    /// Dispatches an action command and returns its outcome.
    ///
    /// # Errors
    ///
    /// Returns [`StepError`] when the command cannot be dispatched or the
    /// action fails to run.
    fn dispatch(
        &self,
        command: ExecuteActionCommand<dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError>;
}
