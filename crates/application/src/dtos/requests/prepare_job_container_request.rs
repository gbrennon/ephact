use std::path::{Path, PathBuf};

/// Describes the repository and generic container image used to prepare one job.
pub struct PrepareJobContainerRequest {
    job_id: String,
    container_image: Option<String>,
    repo_path: PathBuf,
    allow_repo_writes: bool,
}

impl PrepareJobContainerRequest {
    /// Creates a job container preparation request.
    pub fn new(
        job_id: String,
        container_image: Option<String>,
        repo_path: PathBuf,
        allow_repo_writes: bool,
    ) -> Self {
        Self {
            job_id,
            container_image,
            repo_path,
            allow_repo_writes,
        }
    }

    /// Returns the job identifier used to name the isolated container.
    pub fn job_id(&self) -> &str {
        &self.job_id
    }

    /// Returns the explicitly requested container image, if present.
    pub fn container_image(&self) -> Option<&str> {
        self.container_image.as_deref()
    }

    /// Returns the host repository path copied into the isolated container.
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    /// Returns whether the container can write directly to the host repository.
    pub fn allow_repo_writes(&self) -> bool {
        self.allow_repo_writes
    }
}
