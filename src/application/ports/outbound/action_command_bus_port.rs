use crate::{
    application::{
        dtos::responses::ExecuteActionResponse, ports::outbound::container_port::ContainerPort,
    },
    domain::{errors::StepError, messages::commands::ExecuteActionCommand},
};

/// Outbound port dispatching action execution commands.
///
/// Dispatches an [`ExecuteActionCommand`] to its command handler and returns
/// the handler's [`ExecuteActionResponse`].
pub trait ActionCommandBusPort: Send + Sync {
    /// Dispatches an action command and returns the handler's outcome.
    fn dispatch<'a>(
        &self,
        command: ExecuteActionCommand<'a, dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError>;
}
