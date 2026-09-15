use crate::{
    application::{
        dtos::responses::ExecutedStepResponse, ports::outbound::container_port::ContainerPort,
    },
    domain::{errors::StepError, messages::commands::ExecuteStepCommand},
};

/// Outbound port for dispatching a step execution command.
///
/// Implementations route an [`ExecuteStepCommand`] to whatever produces its
/// outcome and return the resulting [`ExecutedStepResponse`].
pub trait StepCommandBusPort: Send + Sync {
    /// Dispatches a step command and returns its outcome.
    ///
    /// # Errors
    ///
    /// Returns [`StepError`] when the command cannot be dispatched or the step
    /// fails to run.
    fn dispatch<'a>(
        &self,
        command: ExecuteStepCommand<'a, dyn ContainerPort>,
    ) -> Result<ExecutedStepResponse, StepError>;
}
