use std::path::Path;

/// Request DTO for the
/// [`PrepareJobContainerPort`](crate::application::ports::inbound::prepare_job_container_port::PrepareJobContainerPort)
/// inbound port.
pub struct PrepareJobContainerRequest<'a> {
    job_id: &'a str,
    runs_on: Option<&'a str>,
    repo_path: &'a Path,
    allow_repo_writes: bool,
}

impl<'a> PrepareJobContainerRequest<'a> {
    /// Creates a new request.
    pub fn new(
        job_id: &'a str,
        runs_on: Option<&'a str>,
        repo_path: &'a Path,
        allow_repo_writes: bool,
    ) -> Self {
        Self {
            job_id,
            runs_on,
            repo_path,
            allow_repo_writes,
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
    pub fn allow_repo_writes(&self) -> bool {
        self.allow_repo_writes
    }
}
