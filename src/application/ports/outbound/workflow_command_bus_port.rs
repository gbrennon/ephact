use std::error::Error;

use crate::{
    application::dtos::responses::WorkflowExecutionResponse,
    domain::messages::commands::ExecuteWorkflowCommand,
};

/// Outbound port for dispatching a workflow execution command.
///
/// Implementations route an [`ExecuteWorkflowCommand`] to whatever produces its
/// outcome and return the resulting [`WorkflowExecutionResponse`].
pub trait WorkflowCommandBusPort: Send + Sync {
    /// Dispatches a workflow command and returns its outcome.
    ///
    /// # Errors
    ///
    /// Returns a boxed [`Error`] when the command cannot be dispatched or its
    /// execution fails.
    fn dispatch(
        &self,
        command: ExecuteWorkflowCommand,
    ) -> Result<WorkflowExecutionResponse, Box<dyn Error>>;
}
