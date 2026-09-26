use std::path::{Path, PathBuf};

/// Request DTO for the
/// [`DetectWorkflowFilePort`](crate::ports::inbound::detect_workflow_file_port::DetectWorkflowFilePort)
/// inbound port.
pub struct DetectWorkflowFileRequest {
    /// Path to the repository whose workflow is detected.
    repo_path: PathBuf,
}

impl DetectWorkflowFileRequest {
    /// Creates a new request.
    pub fn new(repo_path: PathBuf) -> Self {
        Self { repo_path }
    }

    /// Path to the repository whose workflow is detected.
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }
}
