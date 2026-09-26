use std::path::{Path, PathBuf};

/// Request DTO for the
/// [`ListWorkflowDirectoryPort`](crate::ports::inbound::list_workflow_directory_port::ListWorkflowDirectoryPort)
/// inbound port.
pub struct ListWorkflowDirectoryRequest {
    /// Directory whose workflow files are listed.
    directory: PathBuf,
}

impl ListWorkflowDirectoryRequest {
    /// Creates a new request.
    pub fn new(directory: PathBuf) -> Self {
        Self { directory }
    }

    /// Directory whose workflow files are listed.
    pub fn directory(&self) -> &Path {
        &self.directory
    }
}
