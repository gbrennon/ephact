use std::path::{Path, PathBuf};

pub struct CopyRepositoryToContainerRequest {
    repo_path: PathBuf,
    container_path: String,
}

impl CopyRepositoryToContainerRequest {
    pub fn new(repo_path: PathBuf, container_path: String) -> Self {
        Self {
            repo_path,
            container_path,
        }
    }

    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    pub fn container_path(&self) -> &str {
        &self.container_path
    }
}
