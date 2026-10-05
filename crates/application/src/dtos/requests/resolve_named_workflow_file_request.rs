use std::path::{Path, PathBuf};

/// Request data for the outbound operation.
/// The request supplies the workflow name and repository path.
pub struct ResolveNamedWorkflowFileRequest {
    /// Workflow the run was asked to execute, as named on the command line.
    workflow_name: String,
    /// Path to the repository the workflow is looked up in.
    repo_path: PathBuf,
}

impl ResolveNamedWorkflowFileRequest {
    /// Creates a new request.
    pub fn new(workflow_name: String, repo_path: PathBuf) -> Self {
        Self {
            workflow_name,
            repo_path,
        }
    }

    /// Workflow the run was asked to execute, as named on the command line.
    pub fn workflow_name(&self) -> &str {
        &self.workflow_name
    }

    /// Path to the repository the workflow is looked up in.
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }
}
