use std::path::Path;

/// Request DTO for the
/// [`DetectWorkflowFilePort`](crate::application::ports::inbound::detect_workflow_file_port::DetectWorkflowFilePort)
/// inbound port.
pub struct DetectWorkflowFileRequest<'a> {
    /// Path to the repository whose workflow is detected.
    pub repo_path: &'a Path,
}

impl<'a> DetectWorkflowFileRequest<'a> {
    /// Creates a new request.
    pub fn new(repo_path: &'a Path) -> Self {
        Self { repo_path }
    }

    /// Path to the repository whose workflow is detected.
    pub fn repo_path(&self) -> &'a Path {
        self.repo_path
    }
}
