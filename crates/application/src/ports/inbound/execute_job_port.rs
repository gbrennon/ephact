use crate::{
    domain::{aggregates::Workflow, entities::JobRun},
    dtos::{requests::ExecuteJobRequest, responses::JobExecutionResponse},
    errors::ExecuteJobError,
};

/// Inbound port for running one planned job.
pub trait ExecuteJobPort: Send + Sync {
    /// Runs every step of the job inside a fresh container.
    /// The response contains each step summary and the execution container
    /// name.
    ///
    /// # Errors
    ///
    /// Returns [`ExecuteJobError`] if job preparation or execution cannot be
    /// completed.
    fn execute(
        &self,
        request: ExecuteJobRequest,
        run: &JobRun,
        workflow: &Workflow,
    ) -> Result<JobExecutionResponse, ExecuteJobError>;
}
