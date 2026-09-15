use crate::application::ports::inbound::execute_job_port::ExecuteJobPort;
use std::error::Error;

use crate::application::dtos::requests::ExecuteJobRequest;
use crate::application::dtos::responses::JobExecutionResponse;
use crate::domain::entities::JobRun;
use crate::domain::messages::commands::ExecuteJobCommand;
use crate::domain::services::evaluation_context_mapper::EvaluationContextMapper;

/// Infrastructure command handler that processes `ExecuteJobCommand`.
pub struct JobCommandHandler {
    executor: Box<dyn ExecuteJobPort>,
}

impl JobCommandHandler {
    pub fn new(executor: Box<dyn ExecuteJobPort>) -> Self {
        Self { executor }
    }

    pub fn handle(&self, cmd: ExecuteJobCommand) -> Result<JobExecutionResponse, Box<dyn Error>> {
        let (job, job_id, workflow, repo_path, context, run_id, allow_repo_writes) =
            cmd.into_parts();
        let run = JobRun::new(workflow.name().map(str::to_string), job_id, job, None);

        let req = ExecuteJobRequest::new(
            repo_path,
            EvaluationContextMapper::to_parts(&context),
            run_id,
            allow_repo_writes,
        );
        Ok(self.executor.execute(req, &run, &workflow)?)
    }
}
