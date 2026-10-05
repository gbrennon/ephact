use crate::{
    dtos::{requests::ExecuteWorkflowRequest, responses::WorkflowExecutionResponse},
    errors::ExecuteWorkflowError,
};

/// Parses, plans, and executes workflow content.
pub trait ExecuteWorkflowPort: Send + Sync {
    /// Loads the workflow from [`ExecuteWorkflowRequest`] and executes its
    /// planned jobs.
    ///
    /// The response includes the workflow name, job summaries, container names,
    /// and overall success.
    ///
    /// # Errors
    ///
    /// Returns [`ExecuteWorkflowError`] when the workflow cannot be loaded,
    /// planned, or executed.
    fn execute(
        &self,
        request: ExecuteWorkflowRequest,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError>;
}
