use crate::{
    application::{
        dtos::responses::ExecuteActionResponse, ports::outbound::container_port::ContainerPort,
    },
    domain::{errors::StepError, messages::commands::ExecuteActionCommand},
};

/// Outbound port for dispatching action execution commands.
///
/// Implementations route an [`ExecuteActionCommand`] to whatever produces its
/// outcome and return the resulting [`ExecuteActionResponse`]. The command
/// carries a type-erased container handle so command buses remain object-safe
/// and can be composed through `Box<dyn ActionCommandBusPort>`.
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
