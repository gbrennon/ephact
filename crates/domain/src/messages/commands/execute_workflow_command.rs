use crate::{
    entities::repository::Repository, messages::commands::command::Command,
    value_objects::workflow_run_config::WorkflowRunConfig,
};

/// Command representing the intention to execute a workflow.
///
/// Carries the already-resolved workflow definition content and domain objects only:
/// no filesystem paths, handles, or infrastructure references are exposed here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteWorkflowCommand {
    workflow_content: String,
    config: WorkflowRunConfig,
    repository: Repository,
    run_id: String,
    allow_repo_writes: bool,
    workflow_file_name: Option<String>,
}

impl ExecuteWorkflowCommand {
    pub fn new(
        workflow_content: String,
        config: WorkflowRunConfig,
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
            workflow_file_name: None,
        }
    }

    pub fn workflow_content(&self) -> &str {
        &self.workflow_content
    }

    pub fn config(&self) -> &WorkflowRunConfig {
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

    pub fn with_workflow_file_name(mut self, file_name: impl Into<String>) -> Self {
        self.workflow_file_name = Some(file_name.into());
        self
    }

    pub fn workflow_file_name(&self) -> Option<&str> {
        self.workflow_file_name.as_deref()
    }

    pub fn into_parts(
        self,
    ) -> (
        String,
        WorkflowRunConfig,
        Repository,
        String,
        bool,
        Option<String>,
    ) {
        (
            self.workflow_content,
            self.config,
            self.repository,
            self.run_id,
            self.allow_repo_writes,
            self.workflow_file_name,
        )
    }
}

impl Command for ExecuteWorkflowCommand {}
