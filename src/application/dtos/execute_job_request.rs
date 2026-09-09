use std::path::Path;

use crate::domain::{expression::EvalContext, planner::Run, workflow::Workflow};

/// Request DTO for the
/// [`ExecuteJobPort`](crate::application::ports::inbound::execute_job_port::ExecuteJobPort)
/// inbound port.
pub struct ExecuteJobRequest<'a> {
    /// Planned job to run.
    pub run: &'a Run,
    /// Workflow the job belongs to.
    pub workflow: &'a Workflow,
    /// Repository directory the run executes against.
    pub repo_path: &'a Path,
    /// Context the job's steps are evaluated against.
    pub context: &'a EvalContext,
}

impl<'a> ExecuteJobRequest<'a> {
    /// Creates a new request.
    pub fn new(
        run: &'a Run,
        workflow: &'a Workflow,
        repo_path: &'a Path,
        context: &'a EvalContext,
    ) -> Self {
        Self {
            run,
            workflow,
            repo_path,
            context,
        }
    }

    /// Planned job to run.
    pub fn run(&self) -> &'a Run {
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
    pub fn context(&self) -> &'a EvalContext {
        self.context
    }
}
