use std::path::{Path, PathBuf};

/// Request data for preparing a job container.
pub struct PrepareJobContainerRequest {
    job_id: String,
    runs_on: Option<String>,
    repo_path: PathBuf,
    allow_repo_writes: bool,
}

impl PrepareJobContainerRequest {
    /// Creates a new request.
    pub fn new(
        job_id: String,
        runs_on: Option<String>,
        repo_path: PathBuf,
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
    pub fn job_id(&self) -> &str {
        &self.job_id
    }

    /// Runner label the job declared, when it declared one.
    pub fn runs_on(&self) -> Option<&str> {
        self.runs_on.as_deref()
    }

    /// Repository directory mounted into the container as the workspace.
    pub fn repo_path(&self) -> &Path {
        &self.repo_path
    }

    pub fn allow_repo_writes(&self) -> bool {
        self.allow_repo_writes
    }
}
