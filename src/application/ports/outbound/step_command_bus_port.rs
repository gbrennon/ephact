use crate::{
    application::{
        dtos::responses::ExecutedStepResponse, ports::outbound::container_port::ContainerPort,
    },
    domain::{errors::StepError, messages::commands::ExecuteStepCommand},
};

/// Outbound port dispatching step execution commands.
///
/// Dispatches an [`ExecuteStepCommand`] to its command handler and returns the
/// handler's [`ExecutedStepResponse`].
pub trait StepCommandBusPort: Send + Sync {
    /// Dispatches a step command and returns the handler's outcome.
    fn dispatch<'a>(
        &self,
        command: ExecuteStepCommand<'a, dyn ContainerPort>,
    ) -> Result<ExecutedStepResponse, StepError>;
}
