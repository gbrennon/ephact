use crate::{
    domain::messages::commands::ExecuteWorkflowPayload, dtos::responses::WorkflowExecutionResponse,
    errors::ExecuteWorkflowError,
};

/// Publishes workflow execution commands.
pub trait WorkflowCommandPublisherPort: Send + Sync {
    /// Routes `command` to a workflow handler and returns its execution
    /// response.
    ///
    /// # Errors
    ///
    /// Returns [`ExecuteWorkflowError`] when dispatch or workflow execution
    /// fails.
    fn publish(
        &self,
        command: ExecuteWorkflowPayload,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError>;
}
