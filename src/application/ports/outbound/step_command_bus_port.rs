use crate::{
    application::{
        dtos::responses::ExecutedStepResponse, ports::outbound::container_port::ContainerPort,
    },
    domain::{errors::StepError, messages::commands::ExecuteStepCommand},
};

/// Outbound port for dispatching step execution commands.
///
/// Implementations route an [`ExecuteStepCommand`] to whatever produces its
/// outcome and return the resulting [`ExecutedStepResponse`]. The command
/// carries a type-erased container handle so command buses remain object-safe
/// and can be composed through `Box<dyn StepCommandBusPort>`.
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
