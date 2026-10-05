use std::path::{Path, PathBuf};

/// Request data for the outbound operation.
/// The request identifies the directory whose workflow files are listed.
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
