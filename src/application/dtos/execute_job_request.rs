use std::path::Path;

use crate::domain::{aggregates::Workflow, entities::JobRun, value_objects::EvaluationContext};

/// Request DTO for the
/// [`ExecuteJobPort`](crate::application::ports::inbound::execute_job_port::ExecuteJobPort)
/// inbound port.
pub struct ExecuteJobRequest<'a> {
    /// Planned job to run.
    run: &'a JobRun,
    /// Workflow the job belongs to.
    workflow: &'a Workflow,
    /// Repository directory the run executes against.
    repo_path: &'a Path,
    /// Context the job's steps are evaluated against.
    context: &'a EvaluationContext,
}

impl<'a> ExecuteJobRequest<'a> {
    /// Creates a new request.
    pub fn new(
        run: &'a JobRun,
        workflow: &'a Workflow,
        repo_path: &'a Path,
        context: &'a EvaluationContext,
    ) -> Self {
        Self {
            run,
            workflow,
            repo_path,
            context,
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
}
