use crate::{
    application::{
        dtos::{requests::ExecuteJobRequest, responses::JobExecutionResponse},
        errors::ExecuteJobError,
    },
    domain::{aggregates::Workflow, entities::JobRun},
};

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
