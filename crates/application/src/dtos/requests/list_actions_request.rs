use std::path::{Path, PathBuf};

/// Request DTO for listing actions referenced across a repository's workflows.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListActionsRequest {
    repository_path: PathBuf,
    repository_name: String,
}

impl ListActionsRequest {
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
