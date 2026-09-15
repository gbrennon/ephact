use crate::application::dtos::requests::ExecuteJobRequest;
use crate::application::dtos::responses::JobExecutionResponse;
use crate::application::errors::ExecuteJobError;
use crate::domain::aggregates::Workflow;
use crate::domain::entities::JobRun;

/// Inbound port for running one planned job.
pub trait ExecuteJobPort: Send + Sync {
    /// Runs every step of the job inside a fresh container.
    fn execute(
        &self,
        request: ExecuteJobRequest,
        run: &JobRun,
        workflow: &Workflow,
    ) -> Result<JobExecutionResponse, ExecuteJobError>;
}
