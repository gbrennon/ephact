use crate::application::dtos::requests::ExecuteJobRequest;
use crate::application::dtos::responses::JobExecutionResponse;

/// Inbound port for running one planned job.
pub trait ExecuteJobPort: Send + Sync {
    /// Runs every step of the job inside a fresh container.
    fn execute(
        &self,
        request: ExecuteJobRequest<'_>,
    ) -> Result<JobExecutionResponse, Box<dyn std::error::Error>>;
}
