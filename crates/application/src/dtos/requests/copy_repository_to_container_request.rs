use std::path::{Path, PathBuf};

/// Inputs for copying a repository path to a container path.
pub struct CopyRepositoryToContainerRequest {
    repo_path: PathBuf,
    container_path: String,
}

impl CopyRepositoryToContainerRequest {
    /// Creates inputs from a repository path and a container path.
    pub fn new(repo_path: PathBuf, container_path: String) -> Self {
        Self {
            repo_path,
            container_path,
        }
    }

    /// Returns the repository path.
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    /// Returns the container path.
    pub fn container_path(&self) -> &str {
        &self.container_path
    }
}
