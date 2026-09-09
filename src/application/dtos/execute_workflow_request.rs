use std::path::Path;

use crate::domain::expression::EvalContext;

/// Infrastructure-facing request carrying already-resolved workflow content.
pub struct ExecuteWorkflowRequest<'a> {
    workflow_content: &'a str,
    repo_path: &'a Path,
    context: &'a EvalContext,
}

impl<'a> ExecuteWorkflowRequest<'a> {
    pub fn new(workflow_content: &'a str, repo_path: &'a Path, context: &'a EvalContext) -> Self {
        Self {
            workflow_content,
            repo_path,
            context,
        }
    }

    pub fn workflow_content(&self) -> &'a str {
        self.workflow_content
    }

    pub fn repo_path(&self) -> &'a Path {
        self.repo_path
    }

    pub fn context(&self) -> &'a EvalContext {
        self.context
    }
}
