use std::path::Path;

use crate::domain::{aggregates::Workflow, entities::JobRun, value_objects::EvaluationContext};

/// Request DTO for the
/// [`ExecuteJobPort`](crate::application::ports::inbound::execute_job_port::ExecuteJobPort)
/// inbound port.
pub struct ExecuteJobRequest<'a> {
    run: &'a JobRun,
    workflow: &'a Workflow,
    repo_path: &'a Path,
    context: &'a EvaluationContext,
    run_id: &'a str,
    allow_repo_writes: bool,
}

impl<'a> ExecuteJobRequest<'a> {
    /// Creates a new request.
    pub fn new(
        run: &'a JobRun,
        workflow: &'a Workflow,
        repo_path: &'a Path,
        context: &'a EvaluationContext,
        run_id: &'a str,
        allow_repo_writes: bool,
    ) -> Self {
        Self {
            run,
            workflow,
            repo_path,
            context,
            run_id,
            allow_repo_writes,
        }
    }

    /// Planned job to run.
    pub fn run(&self) -> &'a JobRun {
        self.run
    }

    /// Workflow the job belongs to.
    pub fn workflow(&self) -> &'a Workflow {
        self.workflow
    }

    /// Repository directory the run executes against.
    pub fn repo_path(&self) -> &'a Path {
        self.repo_path
    }

    /// Context the job's steps are evaluated against.
    pub fn context(&self) -> &'a EvaluationContext {
        self.context
    }

    pub fn run_id(&self) -> &'a str {
        self.run_id
    }

    pub fn allow_repo_writes(&self) -> bool {
        self.allow_repo_writes
    }
}
