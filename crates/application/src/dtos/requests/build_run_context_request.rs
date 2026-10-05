use crate::domain::{Repository, WorkflowRunConfig};

/// Request data for the outbound operation.
/// The context is built from the run configuration and repository.
pub struct BuildRunContextRequest {
    config: WorkflowRunConfig,
    repository: Repository,
}

impl BuildRunContextRequest {
    pub fn new(config: WorkflowRunConfig, repository: Repository) -> Self {
        Self { config, repository }
    }

    pub fn config(&self) -> &WorkflowRunConfig {
        &self.config
    }

    pub fn repository(&self) -> &Repository {
        &self.repository
    }
}
