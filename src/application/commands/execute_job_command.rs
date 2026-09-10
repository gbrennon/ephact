use std::path::PathBuf;

use crate::domain::{aggregates::Workflow, entities::Job, value_objects::EvaluationContext};

/// Command representing the intention to execute one job of a workflow.
///
/// Published by the workflow coordination service once the execution plan is
/// known, and handled by the job command handler.
#[derive(Debug, Clone)]
pub struct ExecuteJobCommand {
    job: Job,
    job_id: String,
    workflow: Workflow,
    repo_path: PathBuf,
    context: EvaluationContext,
}

impl ExecuteJobCommand {
    pub fn new(
        job: Job,
        job_id: String,
        workflow: Workflow,
        repo_path: PathBuf,
        context: EvaluationContext,
    ) -> Self {
        Self {
            job,
            job_id,
            workflow,
            repo_path,
            context,
        }
    }

    pub fn job(&self) -> &Job {
        &self.job
    }

    pub fn job_id(&self) -> &str {
        &self.job_id
    }

    pub fn workflow(&self) -> &Workflow {
        &self.workflow
    }

    pub fn repo_path(&self) -> &PathBuf {
        &self.repo_path
    }

    pub fn context(&self) -> &EvaluationContext {
        &self.context
    }

    pub fn into_parts(self) -> (Job, String, Workflow, PathBuf, EvaluationContext) {
        (
            self.job,
            self.job_id,
            self.workflow,
            self.repo_path,
            self.context,
        )
    }
}
