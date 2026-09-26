use std::path::{Path, PathBuf};

/// Request DTO for listing workflows in a repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListWorkflowsRequest {
    repository_path: PathBuf,
    repository_name: String,
}

impl ListWorkflowsRequest {
    /// Creates a request from primitive repository data.
    pub fn new(repository_path: PathBuf, repository_name: String) -> Self {
        Self {
            repository_path,
            repository_name,
        }
    }

    /// Returns the repository path.
    pub fn repository_path(&self) -> &Path {
        &self.repository_path
    }

    /// Returns the repository name.
    pub fn repository_name(&self) -> &str {
        &self.repository_name
    }
}
