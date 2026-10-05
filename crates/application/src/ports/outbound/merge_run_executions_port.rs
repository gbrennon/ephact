use crate::dtos::{requests::MergeRunExecutionsRequest, responses::WorkflowExecutionResponse};

/// Combines workflow execution responses into one response when requested.
pub trait MergeRunExecutionsPort: Send + Sync {
    /// If `all_workflows` is false, returns the first execution unchanged.
    /// Otherwise, prefixes job names with their workflow name, combines the
    /// container names, and reports success only when every execution succeeds.
    ///
    /// # Errors
    ///
    /// Returns an error when a single-workflow merge has no execution.
    fn merge(
        &self,
        request: MergeRunExecutionsRequest,
    ) -> Result<WorkflowExecutionResponse, Box<dyn std::error::Error>>;
}
