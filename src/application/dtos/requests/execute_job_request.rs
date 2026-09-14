use std::path::Path;

use crate::domain::{aggregates::Workflow, entities::JobRun, value_objects::EvaluationContext};

pub struct ExecuteJobRequest<'a> {
    run: &'a JobRun,
    workflow: &'a Workflow,
    repo_path: &'a Path,
    context: &'a EvaluationContext,
    run_id: &'a str,
    allow_repo_writes: bool,
}

pub struct ExecuteJobRequestInput<'a> {
    run: &'a JobRun,
    workflow: &'a Workflow,
    execution: ExecuteJobExecutionInput<'a>,
}

impl<'a> ExecuteJobRequestInput<'a> {
    pub fn new(
        run: &'a JobRun,
        workflow: &'a Workflow,
        execution: ExecuteJobExecutionInput<'a>,
    ) -> Self {
        Self {
            run,
            workflow,
            execution,
        }
    }
}

pub struct ExecuteJobExecutionInput<'a> {
    repo_path: &'a Path,
    context: &'a EvaluationContext,
    run_id: &'a str,
    allow_repo_writes: bool,
}

impl<'a> ExecuteJobExecutionInput<'a> {
    pub fn new(
        repo_path: &'a Path,
        context: &'a EvaluationContext,
        run_id: &'a str,
        allow_repo_writes: bool,
    ) -> Self {
        Self {
            repo_path,
            context,
            run_id,
            allow_repo_writes,
        }
    }
}

impl<'a> ExecuteJobRequest<'a> {
    pub fn new(input: ExecuteJobRequestInput<'a>) -> Self {
        let ExecuteJobRequestInput {
            run,
            workflow,
            execution:
                ExecuteJobExecutionInput {
                    repo_path,
                    context,
                    run_id,
                    allow_repo_writes,
                },
        } = input;
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
