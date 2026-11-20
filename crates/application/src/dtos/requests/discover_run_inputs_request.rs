use crate::domain::{Repository, value_objects::WorkflowRunConfig};

/// Inputs for discovering run inputs from a workflow run configuration and a repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiscoverRunInputsRequest {
    config: WorkflowRunConfig,
    repository: Repository,
    action_ref: Option<String>,
}

impl DiscoverRunInputsRequest {
    /// Creates inputs from a workflow run configuration and a repository.
    pub fn new(config: WorkflowRunConfig, repository: Repository) -> Self {
        Self {
            config,
            repository,
            action_ref: None,
        }
    }

    /// Creates inputs for a standalone local action and a repository.
    pub fn for_action(action_ref: String, repository: Repository) -> Self {
        Self {
            config: WorkflowRunConfig::new(),
            repository,
            action_ref: Some(action_ref),
        }
    }

    /// Returns the workflow run configuration.
    pub fn config(&self) -> &WorkflowRunConfig {
        &self.config
    }

    /// Returns the repository.
    pub fn repository(&self) -> &Repository {
        &self.repository
    }

    /// Returns the standalone action reference, when discovery targets an action.
    pub fn action_ref(&self) -> Option<&str> {
        self.action_ref.as_deref()
    }
}
