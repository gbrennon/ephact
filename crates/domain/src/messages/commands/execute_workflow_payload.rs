use crate::{
    entities::repository::Repository, messages::Message,
    value_objects::workflow_run_config::WorkflowRunConfig,
};

/// Data describing the intention to execute one workflow.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecuteWorkflowPayload {
    workflow_content: String,
    config: WorkflowRunConfig,
    repository: Repository,
    run_id: String,
    allow_repo_writes: bool,
    workflow_file_name: Option<String>,
}

impl ExecuteWorkflowPayload {
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

    pub fn with_workflow_file_name(self, file_name: impl Into<String>) -> Self {
        Self {
            workflow_file_name: Some(file_name.into()),
            ..self
        }
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

impl Message for ExecuteWorkflowPayload {}

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

    fn payload_for_test() -> ExecuteWorkflowPayload {
        ExecuteWorkflowPayload::new(
            "content".into(),
            WorkflowRunConfig::new(),
            repository_for_test(),
            "run-1".into(),
            true,
        )
    }

    #[test]
    fn new_exposes_every_field() {
        let payload = payload_for_test();

        assert_eq!(payload.workflow_content(), "content");
        assert_eq!(payload.config(), &WorkflowRunConfig::new());
        assert_eq!(payload.repository(), &repository_for_test());
        assert_eq!(payload.run_id(), "run-1");
        assert!(payload.allow_repo_writes());
    }

    #[test]
    fn into_parts_returns_owned_fields() {
        let (content, config, repository, run_id, allow_repo_writes, workflow_file_name) =
            payload_for_test().into_parts();

        assert_eq!(content, "content");
        assert_eq!(config, WorkflowRunConfig::new());
        assert_eq!(repository, repository_for_test());
        assert_eq!(run_id, "run-1");
        assert!(allow_repo_writes);
        assert_eq!(workflow_file_name, None);
    }

    #[test]
    fn clone_and_equality_hold() {
        let payload = payload_for_test();

        assert_eq!(payload.clone(), payload);
    }
}
