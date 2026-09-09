use crate::domain::{Repository, value_objects::ActRunConfig};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoverRunInputsRequest {
    config: ActRunConfig,
    repository: Repository,
}

impl DiscoverRunInputsRequest {
    pub fn new(config: ActRunConfig, repository: Repository) -> Self {
        Self { config, repository }
    }
    pub fn config(&self) -> &ActRunConfig {
        &self.config
    }
    pub fn repository(&self) -> &Repository {
        &self.repository
    }
}
