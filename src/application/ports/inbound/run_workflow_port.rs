use std::error::Error;

use crate::application::dtos::requests::RunWorkflowRequest;
use crate::application::dtos::responses::RunSummaryResponse;

/// Inbound port representing the entrypoint to run a single workflow in a repository.
pub trait RunWorkflowPort {
    /// Executes a single workflow (specific or detected) in the repository.
    fn execute(&self, request: RunWorkflowRequest) -> Result<RunSummaryResponse, Box<dyn Error>>;
}
