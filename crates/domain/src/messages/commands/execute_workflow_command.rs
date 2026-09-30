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

    pub fn into_parts(self) -> (String, WorkflowRunConfig, Repository, String, bool) {
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

#[cfg(test)]
mod tests {
    use std::{env, path::PathBuf};

    use super::*;
    use crate::value_objects::{RepoPath, RepositoryName};

    fn repository_for_test() -> Repository {
        let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("../..");
        Repository::new(
            RepoPath::new(root).unwrap(),
            RepositoryName::new("test-repo".into()).unwrap(),
        )
    }

    fn command_for_test() -> ExecuteWorkflowCommand {
        ExecuteWorkflowCommand::new(
            "content".into(),
            WorkflowRunConfig::new(),
            repository_for_test(),
            "run-1".into(),
            true,
        )
    }

    #[test]
    fn new_exposes_every_field() {
        let command = command_for_test();

        assert_eq!(command.workflow_content(), "content");
        assert_eq!(command.config(), &WorkflowRunConfig::new());
        assert_eq!(command.repository(), &repository_for_test());
        assert_eq!(command.run_id(), "run-1");
        assert!(command.allow_repo_writes());
    }

    #[test]
    fn into_parts_returns_owned_fields() {
        let (content, config, repository, run_id, allow_repo_writes) =
            command_for_test().into_parts();

        assert_eq!(content, "content");
        assert_eq!(config, WorkflowRunConfig::new());
        assert_eq!(repository, repository_for_test());
        assert_eq!(run_id, "run-1");
        assert!(allow_repo_writes);
    }

    #[test]
    fn clone_and_equality_hold() {
        let command = command_for_test();

        assert_eq!(command.clone(), command);
    }
}
