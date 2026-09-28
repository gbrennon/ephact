use crate::{
    dtos::{requests::RunAllWorkflowsRequest, responses::RunSummaryResponse},
    errors::ApplicationError,
};

/// Inbound port representing the entrypoint to run all workflows in a repository.
pub trait RunAllWorkflowsPort {
    /// Executes all workflows found in the repository according to configuration.
    fn execute(
        &self,
        request: RunAllWorkflowsRequest,
    ) -> Result<RunSummaryResponse, ApplicationError>;
}
