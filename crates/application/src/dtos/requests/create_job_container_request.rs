use std::path::{Path, PathBuf};

/// Request DTO for the
/// [`CreateJobContainerPort`](crate::ports::inbound::create_job_container_port::CreateJobContainerPort)
/// inbound port.
pub struct CreateJobContainerRequest {
    image: String,
    container_name: String,
    legacy_container_name: String,
    repo_path: PathBuf,
    allow_repo_writes: bool,
}

impl CreateJobContainerRequest {
    /// Creates a new request.
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

    /// Image the container is created from.
    pub fn image(&self) -> &str {
        &self.image
    }

    /// Name the new container is given.
    pub fn container_name(&self) -> &str {
        &self.container_name
    }

    /// Name older releases gave the same job's container.
    pub fn legacy_container_name(&self) -> &str {
        &self.legacy_container_name
    }

    /// Repository directory mounted into the container as the workspace.
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    pub fn allow_repo_writes(&self) -> bool {
        self.allow_repo_writes
    }
}
