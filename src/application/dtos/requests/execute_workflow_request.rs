use std::path::Path;

use crate::domain::value_objects::EvaluationContext;

/// Infrastructure-facing request carrying already-resolved workflow content.
pub struct ExecuteWorkflowRequest<'a> {
    workflow_content: &'a str,
    repo_path: &'a Path,
    context: &'a EvaluationContext,
    run_id: &'a str,
    allow_repo_writes: bool,
}

impl<'a> ExecuteWorkflowRequest<'a> {
    pub fn new(
        workflow_content: &'a str,
        repo_path: &'a Path,
        context: &'a EvaluationContext,
        run_id: &'a str,
        allow_repo_writes: bool,
    ) -> Self {
        Self {
            workflow_content,
            repo_path,
            context,
            run_id,
            allow_repo_writes,
        }
    }

    pub fn workflow_content(&self) -> &'a str {
        self.workflow_content
    }

    pub fn repo_path(&self) -> &'a Path {
        self.repo_path
    }

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
