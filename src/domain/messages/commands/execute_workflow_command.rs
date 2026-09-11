use crate::domain::{
    entities::repository::Repository, messages::commands::command::Command,
    value_objects::act_run_config::ActRunConfig,
};

/// Command representing the intention to execute a workflow.
///
/// Carries the already-resolved workflow definition content and domain objects only:
/// no filesystem paths, handles, or infrastructure references are exposed here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteWorkflowCommand {
    workflow_content: String,
    config: ActRunConfig,
    repository: Repository,
}

impl ExecuteWorkflowCommand {
    pub fn new(workflow_content: String, config: ActRunConfig, repository: Repository) -> Self {
        Self {
            workflow_content,
            config,
            repository,
        }
    }

    pub fn workflow_content(&self) -> &str {
        &self.workflow_content
    }

    pub fn config(&self) -> &ActRunConfig {
        &self.config
    }

    pub fn repository(&self) -> &Repository {
        &self.repository
    }

    pub fn into_parts(self) -> (String, ActRunConfig, Repository) {
        (self.workflow_content, self.config, self.repository)
    }
}

impl Command for ExecuteWorkflowCommand {}
