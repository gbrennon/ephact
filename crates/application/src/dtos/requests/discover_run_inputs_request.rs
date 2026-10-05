use crate::domain::{Repository, value_objects::WorkflowRunConfig};

/// Inputs for discovering run inputs from a workflow run configuration and a repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoverRunInputsRequest {
    config: WorkflowRunConfig,
    repository: Repository,
}

impl DiscoverRunInputsRequest {
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
