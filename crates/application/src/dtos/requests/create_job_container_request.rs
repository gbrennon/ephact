use std::path::{Path, PathBuf};

/// Inputs for creating a job container from image and repository values.
pub struct CreateJobContainerRequest {
    image: String,
    container_name: String,
    legacy_container_name: String,
    repo_path: PathBuf,
    allow_repo_writes: bool,
}

impl CreateJobContainerRequest {
    /// Creates inputs from image, container names, a repository path, and a write policy.
    pub fn new(
        image: String,
        container_name: String,
        legacy_container_name: String,
        repo_path: PathBuf,
        allow_repo_writes: bool,
    ) -> Self {
        Self {
            image,
            container_name,
            legacy_container_name,
            repo_path,
            allow_repo_writes,
        }
    }

    /// Returns the image.
    pub fn image(&self) -> &str {
        &self.image
    }

    /// Returns the container name.
    pub fn container_name(&self) -> &str {
        &self.container_name
    }

    /// Returns the legacy container name.
    pub fn legacy_container_name(&self) -> &str {
        &self.legacy_container_name
    }

    /// Returns the repository path.
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    /// Returns whether repository writes are allowed.
    pub fn allow_repo_writes(&self) -> bool {
        self.allow_repo_writes
    }
}
