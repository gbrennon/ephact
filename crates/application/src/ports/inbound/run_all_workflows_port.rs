use crate::{
    dtos::{requests::RunAllWorkflowsRequest, responses::RunSummaryResponse},
    errors::ApplicationError,
};

/// Inbound port representing the entrypoint to run all workflows in a repository.
pub trait RunAllWorkflowsPort {
    /// Executes all workflows found in the repository according to configuration.
    /// The response contains aggregated job summaries, success, and duration.
    ///
    /// # Errors
    ///
    /// Returns [`ApplicationError`] when the event is missing or repository,
    /// workflow, or execution processing fails.
    fn execute(
        &self,
        request: RunAllWorkflowsRequest,
    ) -> Result<RunSummaryResponse, ApplicationError>;
}
