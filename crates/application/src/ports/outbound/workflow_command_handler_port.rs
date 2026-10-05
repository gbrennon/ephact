use crate::{
    domain::messages::commands::ExecuteWorkflowPayload, dtos::responses::WorkflowExecutionResponse,
    errors::ExecuteWorkflowError,
};

/// Handles workflow execution commands.
pub trait WorkflowCommandHandlerPort: Send + Sync {
    /// Builds the workflow evaluation context, executes `command`, and returns
    /// the workflow and job results.
    ///
    /// # Errors
    ///
    /// Returns [`ExecuteWorkflowError`] when workflow execution fails.
    fn handle(
        &self,
        command: ExecuteWorkflowPayload,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError>;
}
