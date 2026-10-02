use crate::{
    application::{
        dtos::{requests::ExecuteJobRequest, responses::JobExecutionResponse},
        errors::ExecuteJobError,
        ports::{inbound::execute_job_port::ExecuteJobPort, outbound::JobCommandHandlerPort},
    },
    domain::{entities::JobRun, messages::commands::ExecuteJobCommand},
};
/// Infrastructure command handler that processes `ExecuteJobCommand`.
pub struct JobCommandHandler {
    executor: Box<dyn ExecuteJobPort>,
}

impl JobCommandHandler {
    pub fn new(executor: Box<dyn ExecuteJobPort>) -> Self {
        Self { executor }
    }

    pub fn handle(&self, cmd: ExecuteJobCommand) -> Result<JobExecutionResponse, ExecuteJobError> {
        let allow_network = cmd.allow_network();
        let (job, job_id, workflow, repo_path, context, run_id, allow_repo_writes) =
            cmd.into_parts();
        let run = JobRun::new(workflow.name().map(str::to_string), job_id, job, None);

        let req = ExecuteJobRequest::new(repo_path, context, run_id, allow_repo_writes)
            .with_allow_network(allow_network);
        self.executor.execute(req, &run, &workflow)
    }
}
impl JobCommandHandlerPort for JobCommandHandler {
    fn handle(&self, command: ExecuteJobCommand) -> Result<JobExecutionResponse, ExecuteJobError> {
        JobCommandHandler::handle(self, command)
    }
}
