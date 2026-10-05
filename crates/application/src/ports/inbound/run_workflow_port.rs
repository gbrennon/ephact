use std::{future::Future, pin::Pin};

use crate::{
    dtos::{requests::RunWorkflowRequest, responses::RunSummaryResponse},
    errors::ApplicationError,
};

/// Runs one workflow selected from a repository.
pub trait RunWorkflowPort: Send + Sync {
    /// Asynchronously loads and executes the requested workflow for its event,
    /// returning a summary with job results, success, and elapsed duration.
    ///
    /// # Errors
    ///
    /// The returned future resolves to [`ApplicationError`] when the workflow
    /// cannot be selected, its event is unsupported, or execution fails.
    fn execute(
        &self,
        request: RunWorkflowRequest,
    ) -> Pin<Box<dyn Future<Output = Result<RunSummaryResponse, ApplicationError>> + Send + '_>>;
}
