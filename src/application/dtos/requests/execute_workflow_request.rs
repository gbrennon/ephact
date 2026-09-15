use std::path::{Path, PathBuf};

/// Primitive request for executing a parsed workflow.
pub struct ExecuteWorkflowRequest {
    workflow_content: String,
    repo_path: PathBuf,
    context: Vec<(String, String)>,
    run_id: String,
    allow_repo_writes: bool,
}

impl ExecuteWorkflowRequest {
    /// Creates a workflow execution request from owned primitive values.
    pub fn new(
        workflow_content: String,
        repo_path: PathBuf,
        context: Vec<(String, String)>,
        run_id: String,
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

    /// Returns the workflow content.
    pub fn workflow_content(&self) -> &str {
        &self.workflow_content
    }

    /// Returns the repository path.
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    /// Returns named JSON-text context roots.
    pub fn context(&self) -> &[(String, String)] {
        &self.context
    }

    /// Returns the run identity.
    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    /// Returns whether repository writes are allowed.
    pub fn allow_repo_writes(&self) -> bool {
        self.allow_repo_writes
    }
}
