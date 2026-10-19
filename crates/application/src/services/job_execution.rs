use std::error::Error;

use super::{
    execute_job_dependencies::ExecuteJobDependencies, job_execution_events::JobExecutionEvents,
    job_execution_state::JobExecutionState, job_step_execution::JobStepExecution,
};
use crate::{
    domain::{aggregates::Workflow, entities::JobRun},
    dtos::{
        requests::{BuildJobEnvironmentRequest, ExecuteJobRequest, PrepareJobContainerRequest},
        responses::{JobExecutionResponse, PreparedJobContainerResponse},
    },
    errors::ExecuteJobError,
};

/// One invocation of the job execution flow.
pub struct JobExecution<'a> {
    dependencies: &'a ExecuteJobDependencies,
    state: Option<JobExecutionState>,
    prepared: Option<PreparedJobContainerResponse>,
}

impl<'a> JobExecution<'a> {
    /// Creates an execution tied to the dependencies used by one job run.
    pub fn new(dependencies: &'a ExecuteJobDependencies) -> Self {
        Self {
            dependencies,
            state: None,
            prepared: None,
        }
    }

    /// Executes all steps in a job and assembles its response.
    pub fn execute(
        self,
        request: ExecuteJobRequest,
        run: &JobRun,
        workflow: &Workflow,
    ) -> Result<JobExecutionResponse, ExecuteJobError> {
        let mut execution = self;
        execution
            .prepare_execution(&request, run, workflow)
            .map_err(|error| ExecuteJobError::Preparation(error.to_string()))?;
        let (_, _, _, _, _, _, _, event_bus, _) = execution.dependencies.as_parts();
        {
            let prepared = execution
                .prepared
                .as_ref()
                .expect("job execution preparation must create a container");
            JobExecutionEvents::new(event_bus).publish_container_started(&request, prepared);
            let step_execution =
                JobStepExecution::new(execution.dependencies, &request, workflow, run, prepared);
            let state = execution
                .state
                .as_mut()
                .expect("job execution preparation must create state");
            for step in run.job().steps() {
                let continues_after_failure = run.job().continues_after_failure();
                if state.should_skip_step(continues_after_failure) {
                    step_execution.skip(step, state, continues_after_failure);
                } else {
                    step_execution.execute(step, state);
                }
            }
        }
        let container_name = execution
            .prepared
            .take()
            .expect("job execution preparation must create a container")
            .into_container_name();
        let state = execution
            .state
            .take()
            .expect("job execution preparation must create state");
        Ok(state.into_response(run, container_name))
    }

    fn prepare_execution(
        &mut self,
        request: &ExecuteJobRequest,
        run: &JobRun,
        workflow: &Workflow,
    ) -> Result<(), Box<dyn Error>> {
        let (job_environment_builder, container_preparer, _, _, _, _, _, _, _) =
            self.dependencies.as_parts();
        let step_env = job_environment_builder
            .build(
                BuildJobEnvironmentRequest::new(workflow.clone(), run.job().env().clone())
                    .with_context(request.context().clone())
                    .with_event_payload_opt(request.event_payload()),
            )
            .into_env();
        let prepare_request = PrepareJobContainerRequest::new(
            run.job_id().to_string(),
            run.job()
                .container()
                .map(|container| container.image().to_string()),
            request.repo_path().to_path_buf(),
            request.allow_repo_writes(),
        );
        let prepare_request = match request.event_payload() {
            Some(payload) => prepare_request.with_event_payload(payload),
            None => prepare_request,
        };
        let prepared = container_preparer.prepare(prepare_request)?;
        self.state = Some(JobExecutionState::new(step_env));
        self.prepared = Some(prepared);
        Ok(())
    }
}
