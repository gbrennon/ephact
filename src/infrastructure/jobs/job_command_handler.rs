use crate::application::ports::inbound::execute_job_port::ExecuteJobPort;
use std::error::Error;

use crate::application::commands::ExecuteJobCommand;
use crate::{
    application::dtos::{ExecuteJobRequest, JobExecution},
    domain::planner::Run,
};

/// Infrastructure command handler that processes `ExecuteJobCommand`.
pub struct JobCommandHandler {
    executor: Box<dyn ExecuteJobPort>,
}

impl JobCommandHandler {
    pub fn new(executor: Box<dyn ExecuteJobPort>) -> Self {
        Self { executor }
    }

    pub fn handle(&self, cmd: ExecuteJobCommand) -> Result<JobExecution, Box<dyn Error>> {
        let (job, job_id, workflow, repo_path, context) = cmd.into_parts();
        let run = Run::new(workflow.name().map(str::to_string), job_id, job, None);

        let req = ExecuteJobRequest::new(&run, &workflow, &repo_path, &context);
        self.executor.execute(req)
    }
}
