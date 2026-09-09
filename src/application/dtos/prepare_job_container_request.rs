use std::path::Path;

/// Request DTO for the
/// [`PrepareJobContainerPort`](crate::application::ports::inbound::prepare_job_container_port::PrepareJobContainerPort)
/// inbound port.
pub struct PrepareJobContainerRequest<'a> {
    /// Identifier of the job the container is prepared for.
    pub job_id: &'a str,
    /// Runner label the job declared, when it declared one.
    pub runs_on: Option<&'a str>,
    /// Repository directory mounted into the container as the workspace.
    pub repo_path: &'a Path,
}

impl<'a> PrepareJobContainerRequest<'a> {
    /// Creates a new request.
    pub fn new(job_id: &'a str, runs_on: Option<&'a str>, repo_path: &'a Path) -> Self {
        Self {
            job_id,
            runs_on,
            repo_path,
        }
    }

    /// Identifier of the job the container is prepared for.
    pub fn job_id(&self) -> &'a str {
        self.job_id
    }

    /// Runner label the job declared, when it declared one.
    pub fn runs_on(&self) -> Option<&'a str> {
        self.runs_on
    }

    /// Repository directory mounted into the container as the workspace.
    pub fn repo_path(&self) -> &'a Path {
        self.repo_path
    }
}
