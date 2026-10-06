use std::path::{Path, PathBuf};

use crate::domain::{Repository, WorkflowRunConfig};

/// Primitive request for executing one workflow selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunWorkflowRequest {
    repository_path: PathBuf,
    repository_name: String,
    workflow: Option<String>,
    job: Option<String>,
    event: Option<String>,
    inputs: Vec<(String, String)>,
    secrets: Vec<(String, String)>,
    all_workflows: bool,
    allow_repo_writes: bool,
    allow_real_container: bool,
    allow_real_fetcher: bool,
    allow_network: bool,
    run_id: String,
}

impl RunWorkflowRequest {
    /// Creates a request from a repository, run configuration, and run identifier.
    pub fn from_domain(
        repository: &Repository,
        config: &WorkflowRunConfig,
        run_id: impl Into<String>,
    ) -> Self {
        Self {
            repository_path: repository.path().as_path().to_path_buf(),
            repository_name: repository.name().as_str().to_string(),
            workflow: config.workflow().map(|value| value.as_str().to_string()),
            job: config.job().map(|value| value.as_str().to_string()),
            event: config.event().map(|value| value.as_str().to_string()),
            inputs: config
                .inputs()
                .iter()
                .map(|input| (input.key().to_string(), input.value().to_string()))
                .collect(),
            secrets: config
                .secrets()
                .iter()
                .map(|secret| (secret.name().to_string(), secret.value().to_string()))
                .collect(),
            all_workflows: config.all_workflows(),
            allow_repo_writes: config.allow_repo_writes(),
            allow_real_container: config.allow_real_container(),
            allow_real_fetcher: config.allow_real_fetcher(),
            allow_network: config.allow_network(),
            run_id: run_id.into(),
        }
    }

    /// Returns the repository path.
    pub fn repository_path(&self) -> &Path {
        &self.repository_path
    }

    /// Returns the repository name.
    pub fn repository_name(&self) -> &str {
        &self.repository_name
    }

    /// Returns the selected workflow.
    pub fn workflow(&self) -> Option<&str> {
        self.workflow.as_deref()
    }

    /// Returns the selected job.
    pub fn job(&self) -> Option<&str> {
        self.job.as_deref()
    }

    /// Returns the selected event.
    pub fn event(&self) -> Option<&str> {
        self.event.as_deref()
    }

    /// Returns configured input pairs.
    pub fn inputs(&self) -> &[(String, String)] {
        &self.inputs
    }

    /// Returns configured secret pairs.
    pub fn secrets(&self) -> &[(String, String)] {
        &self.secrets
    }

    /// Returns whether all workflows should run.
    pub fn all_workflows(&self) -> bool {
        self.all_workflows
    }

    /// Returns whether repository writes are allowed.
    pub fn allow_repo_writes(&self) -> bool {
        self.allow_repo_writes
    }

    /// Returns whether real container execution is allowed.
    pub fn allow_real_container(&self) -> bool {
        self.allow_real_container
    }

    /// Returns whether real action fetching is allowed.
    pub fn allow_real_fetcher(&self) -> bool {
        self.allow_real_fetcher
    }

    /// Returns whether network access is allowed.
    pub fn allow_network(&self) -> bool {
        self.allow_network
    }

    /// Returns the run identifier.
    pub fn run_id(&self) -> &str {
        &self.run_id
    }
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use crate::{
        domain::{
            entities::Repository,
            value_objects::{
                JobName, RepoPath, RepositoryName, Secret, WorkflowEvent, WorkflowInput,
                WorkflowPath, WorkflowRunConfig,
            },
        },
        dtos::requests::RunWorkflowRequest,
    };

    #[test]
    fn from_domain_copies_configured_values() {
        let repository = Repository::new(
            RepoPath::new(PathBuf::from("workspace")).unwrap(),
            RepositoryName::new("test-repo".to_owned()).unwrap(),
        );
        let config = WorkflowRunConfig::new()
            .with_workflow(WorkflowPath::new(".ci/workflows/ci.yml".to_owned()))
            .with_job(JobName::new("build".to_owned()))
            .with_event(WorkflowEvent::new("push".to_owned()))
            .add_input(WorkflowInput::new(
                "environment".to_owned(),
                "staging".to_owned(),
            ))
            .add_secret(Secret::new("TOKEN".to_owned(), "secret".to_owned()))
            .with_all_workflows(true)
            .with_allow_repo_writes(true)
            .with_allow_real_container(true)
            .with_allow_real_fetcher(true)
            .with_allow_network(true);

        let request = RunWorkflowRequest::from_domain(&repository, &config, "run-1");

        assert_eq!(request.repository_path(), Path::new("workspace"));
        assert_eq!(request.repository_name(), "test-repo");
        assert_eq!(request.workflow(), Some(".ci/workflows/ci.yml"));
        assert_eq!(request.job(), Some("build"));
        assert_eq!(request.event(), Some("push"));
        assert_eq!(
            request.inputs(),
            &[("environment".to_owned(), "staging".to_owned())]
        );
        assert_eq!(
            request.secrets(),
            &[("TOKEN".to_owned(), "secret".to_owned())]
        );
        assert!(request.all_workflows());
        assert!(request.allow_repo_writes());
        assert!(request.allow_real_container());
        assert!(request.allow_real_fetcher());
        assert!(request.allow_network());
        assert_eq!(request.run_id(), "run-1");
    }

    #[test]
    fn from_domain_preserves_empty_optional_values() {
        let repository = Repository::new(
            RepoPath::new(PathBuf::from("workspace")).unwrap(),
            RepositoryName::new("test-repo".to_owned()).unwrap(),
        );
        let config = WorkflowRunConfig::new();

        let request = RunWorkflowRequest::from_domain(&repository, &config, "run-2");

        assert_eq!(request.workflow(), None);
        assert_eq!(request.job(), None);
        assert_eq!(request.event(), None);
        assert!(request.inputs().is_empty());
        assert!(request.secrets().is_empty());
        assert!(!request.all_workflows());
        assert!(!request.allow_repo_writes());
        assert!(!request.allow_real_container());
        assert!(!request.allow_real_fetcher());
        assert!(!request.allow_network());
        assert_eq!(request.run_id(), "run-2");
    }
}
