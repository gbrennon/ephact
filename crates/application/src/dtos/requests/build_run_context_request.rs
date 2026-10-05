use crate::domain::{Repository, WorkflowRunConfig};

/// Inputs for building a run context from a workflow run configuration and a repository.
pub struct BuildRunContextRequest {
    config: WorkflowRunConfig,
    repository: Repository,
}

impl BuildRunContextRequest {
    /// Creates inputs from a workflow run configuration and a repository.
    pub fn new(config: WorkflowRunConfig, repository: Repository) -> Self {
        Self { config, repository }
    }

    /// Returns the workflow run configuration.
    pub fn config(&self) -> &WorkflowRunConfig {
        &self.config
    }

    /// Returns the repository.
    pub fn repository(&self) -> &Repository {
        &self.repository
    }
}
