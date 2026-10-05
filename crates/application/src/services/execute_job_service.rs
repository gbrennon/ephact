use super::{ExecuteJobDependencies, JobExecution};
use crate::{
    domain::{aggregates::Workflow, entities::JobRun},
    dtos::{requests::ExecuteJobRequest, responses::JobExecutionResponse},
    errors::ExecuteJobError,
    ports::inbound::execute_job_port::ExecuteJobPort,
};

/// Application service coordinating the execution of one job.
///
/// The service is the inbound entrypoint; each invocation delegates its
/// per-run state and orchestration to [`JobExecution`].
pub struct ExecuteJobService {
    dependencies: ExecuteJobDependencies,
}

impl ExecuteJobService {
    /// Creates a job execution service from its required dependencies.
    pub fn new(dependencies: ExecuteJobDependencies) -> Self {
        Self { dependencies }
    }
}

impl ExecuteJobPort for ExecuteJobService {
    fn execute(
        &self,
        request: ExecuteJobRequest,
        run: &JobRun,
        workflow: &Workflow,
    ) -> Result<JobExecutionResponse, ExecuteJobError> {
        JobExecution::new(&self.dependencies).execute(request, run, workflow)
    }
}
