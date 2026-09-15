use std::path::{Path, PathBuf};

/// Request DTO for the
/// [`ListAllWorkflowFilesPort`](crate::application::ports::inbound::list_all_workflow_files_port::ListAllWorkflowFilesPort)
/// inbound port.
pub struct ListAllWorkflowFilesRequest {
    /// Path to the repository whose workflow files are listed.
    repo_path: PathBuf,
}

impl ListAllWorkflowFilesRequest {
    /// Creates a new request.
    pub fn new(repo_path: PathBuf) -> Self {
        Self { repo_path }
    }

    /// Path to the repository whose workflow files are listed.
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }
}
