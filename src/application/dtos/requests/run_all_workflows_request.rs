use std::path::{Path, PathBuf};

/// Primitive request for executing every workflow in a repository.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunAllWorkflowsRequest {
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

impl RunAllWorkflowsRequest {
    /// Creates a primitive request from the domain run configuration.
    pub fn from_domain(
        repository: &crate::domain::Repository,
        config: &crate::domain::ActRunConfig,
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
            run_id: config.run_id().to_string(),
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

    /// Returns whether a real container is allowed.
    pub fn allow_real_container(&self) -> bool {
        self.allow_real_container
    }

    /// Returns whether a real fetcher is allowed.
    pub fn allow_real_fetcher(&self) -> bool {
        self.allow_real_fetcher
    }

    /// Returns whether network access is allowed.
    pub fn allow_network(&self) -> bool {
        self.allow_network
    }

    /// Returns the run identity.
    pub fn run_id(&self) -> &str {
        &self.run_id
    }
}
