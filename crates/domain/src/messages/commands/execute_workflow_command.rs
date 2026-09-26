use crate::{
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
    run_id: String,
    allow_repo_writes: bool,
}

impl ExecuteWorkflowCommand {
    pub fn new(
        workflow_content: String,
        config: ActRunConfig,
        repository: Repository,
        run_id: String,
        allow_repo_writes: bool,
    ) -> Self {
        Self {
            workflow_content,
            config,
            repository,
            run_id,
            allow_repo_writes,
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

    pub fn run_id(&self) -> &str {
        &self.run_id
    }

    pub fn allow_repo_writes(&self) -> bool {
        self.allow_repo_writes
    }

    pub fn into_parts(self) -> (String, ActRunConfig, Repository, String, bool) {
        (
            self.workflow_content,
            self.config,
            self.repository,
            self.run_id,
            self.allow_repo_writes,
        )
    }
}

impl Command for ExecuteWorkflowCommand {}
