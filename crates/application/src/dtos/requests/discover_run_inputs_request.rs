use crate::domain::{Repository, value_objects::WorkflowRunConfig};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoverRunInputsRequest {
    config: WorkflowRunConfig,
    repository: Repository,
}

impl DiscoverRunInputsRequest {
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
