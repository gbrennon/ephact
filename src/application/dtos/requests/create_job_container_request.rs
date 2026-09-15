use std::path::Path;

/// Request DTO for the
/// [`CreateJobContainerPort`](crate::application::ports::inbound::create_job_container_port::CreateJobContainerPort)
/// inbound port.
pub struct CreateJobContainerRequest<'a> {
    image: &'a str,
    container_name: &'a str,
    legacy_container_name: &'a str,
    repo_path: &'a Path,
    allow_repo_writes: bool,
}

impl<'a> CreateJobContainerRequest<'a> {
    /// Creates a new request.
    pub fn new(
        image: &'a str,
        container_name: &'a str,
        legacy_container_name: &'a str,
        repo_path: &'a Path,
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
    pub fn image(&self) -> &'a str {
        self.image
    }

    /// Name the new container is given.
    pub fn container_name(&self) -> &'a str {
        self.container_name
    }

    /// Name older releases gave the same job's container.
    pub fn legacy_container_name(&self) -> &'a str {
        self.legacy_container_name
    }

    /// Repository directory mounted into the container as the workspace.
    pub fn repo_path(&self) -> &'a Path {
        self.repo_path
    }

    pub fn allow_repo_writes(&self) -> bool {
        self.allow_repo_writes
    }
}
