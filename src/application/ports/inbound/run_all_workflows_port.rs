use crate::application::dtos::requests::RunAllWorkflowsRequest;
use crate::application::dtos::responses::RunSummaryResponse;
use crate::application::errors::RunAllWorkflowsError;

/// Inbound port representing the entrypoint to run all workflows in a repository.
pub trait RunAllWorkflowsPort {
    /// Executes all workflows found in the repository according to configuration.
    fn execute(
        &self,
        request: RunAllWorkflowsRequest,
    ) -> Result<RunSummaryResponse, RunAllWorkflowsError>;
}
