use std::path::Path;

use crate::{
    application::dtos::requests::ExecuteWorkflowRequest,
    domain::{
        aggregates::Workflow, entities::JobRun, messages::commands::ExecuteJobPayload,
        value_objects::EvaluationContext,
    },
};

/// Immutable values shared while executing one planned job run.
pub struct JobExecutionInput<'a> {
    workflow: &'a Workflow,
    run: &'a JobRun,
    repo_path: &'a Path,
    context: &'a EvaluationContext,
    run_id: &'a str,
    allow_repo_writes: bool,
    event_payload: Option<String>,
    allow_network: bool,
}

impl<'a> JobExecutionInput<'a> {
    /// Builds execution input from a planned run and workflow request.
    pub fn new(
        workflow: &'a Workflow,
        run: &'a JobRun,
        request: &'a ExecuteWorkflowRequest,
        context: &'a EvaluationContext,
    ) -> Self {
        Self {
            workflow,
            run,
            repo_path: request.repo_path(),
            context,
            run_id: request.run_id(),
            allow_repo_writes: request.allow_repo_writes(),
            event_payload: request.event_payload().map(str::to_owned),
            allow_network: request.allow_network(),
        }
    }

    /// Returns the workflow being executed.
    pub fn workflow(&self) -> &Workflow {
        self.workflow
    }

    /// Returns the planned job run.
    pub fn run(&self) -> &JobRun {
        self.run
    }

    /// Returns the evaluation context for the planned job run.
    pub fn context(&self) -> &EvaluationContext {
        self.context
    }

    /// Builds the command payload for the planned job run.
    pub fn execute_job_payload(&self) -> ExecuteJobPayload {
        ExecuteJobPayload::new(
            self.run.job().clone(),
            self.run.job_id().to_string(),
            self.workflow.clone(),
            self.repo_path.to_path_buf(),
            self.context.clone(),
        )
        .with_event_payload_opt(self.event_payload.as_deref())
        .with_run_id(self.run_id.to_string())
        .with_allow_repo_writes(self.allow_repo_writes)
        .with_allow_network(self.allow_network)
    }
}
