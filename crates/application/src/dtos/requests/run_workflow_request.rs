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
    /// Creates a primitive request from the domain run configuration.
    pub fn from_domain(
        repository: &Repository,
        config: &WorkflowRunConfig,
        run_id: impl Into<String>,
    ) -> Self {
        RunRequestBuilder::new()
            .repository_path(repository.path().as_path())
            .repository_name(repository.name().as_str())
            .workflow(config.workflow().map(|value| value.as_str()))
            .job(config.job().map(|value| value.as_str()))
            .event(config.event().map(|value| value.as_str()))
            .inputs(
                config
                    .inputs()
                    .iter()
                    .map(|input| (input.key(), input.value())),
            )
            .secrets(
                config
                    .secrets()
                    .iter()
                    .map(|secret| (secret.name(), secret.value())),
            )
            .all_workflows(config.all_workflows())
            .allow_repo_writes(config.allow_repo_writes())
            .allow_real_container(config.allow_real_container())
            .allow_real_fetcher(config.allow_real_fetcher())
            .allow_network(config.allow_network())
            .run_id(run_id)
            .build()
            .expect("domain values produce a valid run request")
    }
}

/// Incrementally constructs a validated [`RunWorkflowRequest`].
#[derive(Debug, Default)]
pub struct RunRequestBuilder {
    repository_path: Option<PathBuf>,
    repository_name: Option<String>,
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
    run_id: Option<String>,
}

impl RunRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn repository_path(mut self, value: &Path) -> Self {
        self.repository_path = Some(value.to_path_buf());
        self
    }

    pub fn repository_name(mut self, value: impl Into<String>) -> Self {
        self.repository_name = Some(value.into());
        self
    }

    pub fn workflow(mut self, value: Option<impl Into<String>>) -> Self {
        self.workflow = value.map(Into::into);
        self
    }

    pub fn job(mut self, value: Option<impl Into<String>>) -> Self {
        self.job = value.map(Into::into);
        self
    }

    pub fn event(mut self, value: Option<impl Into<String>>) -> Self {
        self.event = value.map(Into::into);
        self
    }

    pub fn inputs<I, K, V>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.inputs = values
            .into_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect();
        self
    }

    pub fn secrets<I, K, V>(mut self, values: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
        K: Into<String>,
        V: Into<String>,
    {
        self.secrets = values
            .into_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect();
        self
    }

    pub fn all_workflows(mut self, value: bool) -> Self {
        self.all_workflows = value;
        self
    }

    pub fn allow_repo_writes(mut self, value: bool) -> Self {
        self.allow_repo_writes = value;
        self
    }

    pub fn allow_real_container(mut self, value: bool) -> Self {
        self.allow_real_container = value;
        self
    }

    pub fn allow_real_fetcher(mut self, value: bool) -> Self {
        self.allow_real_fetcher = value;
        self
    }

    pub fn allow_network(mut self, value: bool) -> Self {
        self.allow_network = value;
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn build(self) -> Result<RunWorkflowRequest, &'static str> {
        Ok(RunWorkflowRequest {
            repository_path: required(self.repository_path, "repository path")?,
            repository_name: required(self.repository_name, "repository name")?,
            workflow: self.workflow,
            job: self.job,
            event: self.event,
            inputs: self.inputs,
            secrets: self.secrets,
            all_workflows: self.all_workflows,
            allow_repo_writes: self.allow_repo_writes,
            allow_real_container: self.allow_real_container,
            allow_real_fetcher: self.allow_real_fetcher,
            allow_network: self.allow_network,
            run_id: required(self.run_id, "run id")?,
        })
    }
}

fn required<T>(value: Option<T>, field: &'static str) -> Result<T, &'static str> {
    value.ok_or(field)
}

#[cfg(test)]
mod run_workflow_request_tests;

impl RunWorkflowRequest {
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
