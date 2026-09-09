use std::path::Path;

/// Request DTO for the
/// [`CreateJobContainerPort`](crate::application::ports::inbound::create_job_container_port::CreateJobContainerPort)
/// inbound port.
pub struct CreateJobContainerRequest<'a> {
    /// Image the container is created from.
    pub image: &'a str,
    /// Name the new container is given.
    pub container_name: &'a str,
    /// Name older releases gave the same job's container.
    pub legacy_container_name: &'a str,
    /// Repository directory mounted into the container as the workspace.
    pub repo_path: &'a Path,
}

impl<'a> CreateJobContainerRequest<'a> {
    /// Creates a new request.
    pub fn new(
        image: &'a str,
        container_name: &'a str,
        legacy_container_name: &'a str,
        repo_path: &'a Path,
    ) -> Self {
        Self {
            image,
            container_name,
            legacy_container_name,
            repo_path,
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
}
