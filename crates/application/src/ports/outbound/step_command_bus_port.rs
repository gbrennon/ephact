use crate::{
    domain::{errors::StepError, messages::commands::ExecuteStepCommand},
    dtos::responses::ExecutedStepResponse,
    ports::outbound::container_port::ContainerPort,
};

/// Dispatches a step execution command and returns its outcome.
pub trait StepCommandBusPort: Send + Sync {
    /// Dispatches a step command and returns its execution outcome.
    ///
    /// # Errors
    ///
    /// Returns [`StepError`] when dispatching or step execution fails.
    fn dispatch(
        &self,
        command: ExecuteStepCommand<dyn ContainerPort>,
    ) -> Result<ExecutedStepResponse, StepError>;
}
