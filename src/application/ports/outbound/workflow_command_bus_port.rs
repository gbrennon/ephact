use std::error::Error;

use crate::{
    application::dtos::responses::WorkflowExecutionResponse,
    domain::messages::commands::ExecuteWorkflowCommand,
};

/// Outbound port dispatching workflow execution commands.
///
/// Dispatches an [`ExecuteWorkflowCommand`] to its command handler and returns
/// the handler's [`WorkflowExecutionResponse`].
pub trait WorkflowCommandBusPort: Send + Sync {
    /// Dispatches a workflow command and returns the handler's outcome.
    fn dispatch(
        &self,
        command: ExecuteWorkflowCommand,
    ) -> Result<WorkflowExecutionResponse, Box<dyn Error>>;
}
