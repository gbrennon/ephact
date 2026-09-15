use uuid::Uuid;

use crate::domain::value_objects::{ActEvent, ActInput, ActJob, ActWorkflow, Secret};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActRunConfig {
    workflow: Option<ActWorkflow>,
    job: Option<ActJob>,
    event: Option<ActEvent>,
    inputs: Vec<ActInput>,
    secrets: Vec<Secret>,
    all_workflows: bool,
    allow_repo_writes: bool,
    allow_real_container: bool,
    allow_real_fetcher: bool,
    allow_network: bool,
    run_id: String,
}
#[derive(Default)]
pub(crate) struct ActRunConfigParts {
    workflow: Option<ActWorkflow>,
    job: Option<ActJob>,
    event: Option<ActEvent>,
    inputs: Vec<ActInput>,
    secrets: Vec<Secret>,
    all_workflows: bool,
    allow_repo_writes: bool,
    allow_real_container: bool,
    allow_real_fetcher: bool,
    allow_network: bool,
    run_id: String,
}
impl ActRunConfigParts {
    pub(crate) fn with_workflow(mut self, workflow: Option<ActWorkflow>) -> Self {
        self.workflow = workflow;
        self
    }

    pub(crate) fn with_job(mut self, job: Option<ActJob>) -> Self {
        self.job = job;
        self
    }

    pub(crate) fn with_event(mut self, event: Option<ActEvent>) -> Self {
        self.event = event;
        self
    }

    pub(crate) fn with_inputs(mut self, inputs: Vec<ActInput>) -> Self {
        self.inputs = inputs;
        self
    }

    pub(crate) fn with_secrets(mut self, secrets: Vec<Secret>) -> Self {
        self.secrets = secrets;
        self
    }

    pub(crate) fn with_all_workflows(mut self, all_workflows: bool) -> Self {
        self.all_workflows = all_workflows;
        self
    }

    pub(crate) fn with_allow_repo_writes(mut self, allow_repo_writes: bool) -> Self {
        self.allow_repo_writes = allow_repo_writes;
        self
    }

    pub(crate) fn with_allow_real_container(mut self, allow_real_container: bool) -> Self {
        self.allow_real_container = allow_real_container;
        self
    }

    pub(crate) fn with_allow_real_fetcher(mut self, allow_real_fetcher: bool) -> Self {
        self.allow_real_fetcher = allow_real_fetcher;
        self
    }

    pub(crate) fn with_allow_network(mut self, allow_network: bool) -> Self {
        self.allow_network = allow_network;
        self
    }

    pub(crate) fn with_run_id(mut self, run_id: String) -> Self {
        self.run_id = run_id;
        self
    }
}

/// Constructors for [`ActRunConfig`].
impl ActRunConfig {
    /// Creates a new config with sensible defaults and a unique run identity.
    pub fn new() -> Self {
        Self {
            workflow: None,
            job: None,
            event: None,
            inputs: Vec::new(),
            secrets: Vec::new(),
            all_workflows: false,
            allow_repo_writes: false,
            allow_real_container: false,
            allow_real_fetcher: false,
            allow_network: false,
            run_id: Uuid::new_v4().to_string(),
        }
    }
}

impl Default for ActRunConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl ActRunConfig {
    pub(crate) fn from_parts(parts: ActRunConfigParts) -> Self {
        Self {
            workflow: parts.workflow,
            job: parts.job,
            event: parts.event,
            inputs: parts.inputs,
            secrets: parts.secrets,
            all_workflows: parts.all_workflows,
            allow_repo_writes: parts.allow_repo_writes,
            allow_real_container: parts.allow_real_container,
            allow_real_fetcher: parts.allow_real_fetcher,
            allow_network: parts.allow_network,
            run_id: parts.run_id,
        }
    }
}

/// Builder API - fluent setters that consume and return `Self`.
impl ActRunConfig {
    /// Sets the workflow file to run.
    pub fn with_workflow(mut self, workflow: ActWorkflow) -> Self {
        self.workflow = Some(workflow);
        self
    }

    /// Sets the specific job to run within the workflow.
    pub fn with_job(mut self, job: ActJob) -> Self {
        self.job = Some(job);
        self
    }

    /// Sets the event to simulate.
    pub fn with_event(mut self, event: ActEvent) -> Self {
        self.event = Some(event);
        self
    }

    /// Adds an input variable.
    pub fn add_input(mut self, input: ActInput) -> Self {
        self.inputs.push(input);
        self
    }

    /// Adds a secret available to `${{ secrets.* }}` expressions.
    pub fn add_secret(mut self, secret: Secret) -> Self {
        self.secrets.push(secret);
        self
    }

    /// Enable running all workflows found in the repository.
    pub fn with_all_workflows(mut self, all_workflows: bool) -> Self {
        self.all_workflows = all_workflows;
        self
    }

    /// Opt into the real container runtime adapter.
    pub fn with_allow_real_container(mut self, allow_real_container: bool) -> Self {
        self.allow_real_container = allow_real_container;
        self
    }

    /// Opt into the real action fetcher that contacts the forge.
    pub fn with_allow_real_fetcher(mut self, allow_real_fetcher: bool) -> Self {
        self.allow_real_fetcher = allow_real_fetcher;
        self
    }

    /// Allow containers to make outbound network requests.
    pub fn with_allow_network(mut self, allow_network: bool) -> Self {
        self.allow_network = allow_network;
        self
    }
    /// Opt into workflow writes to the repository mount.
    pub fn with_allow_repo_writes(mut self, allow_repo_writes: bool) -> Self {
        self.allow_repo_writes = allow_repo_writes;
        self
    }
}

/// Read-only access to each field of [`ActRunConfig`].
impl ActRunConfig {
    /// Returns the workflow, if set.
    pub fn workflow(&self) -> Option<&ActWorkflow> {
        self.workflow.as_ref()
    }

    /// Returns the job, if set.
    pub fn job(&self) -> Option<&ActJob> {
        self.job.as_ref()
    }

    /// Returns the event, if set.
    pub fn event(&self) -> Option<&ActEvent> {
        self.event.as_ref()
    }

    /// Returns all input variables.
    pub fn inputs(&self) -> &[ActInput] {
        &self.inputs
    }

    /// Returns all secrets.
    pub fn secrets(&self) -> &[Secret] {
        &self.secrets
    }

    /// Returns whether to run all workflows.
    pub fn all_workflows(&self) -> bool {
        self.all_workflows
    }

    /// Returns whether the real container runtime was opted into.
    pub fn allow_real_container(&self) -> bool {
        self.allow_real_container
    }

    /// Returns whether the real action fetcher was opted into.
    pub fn allow_real_fetcher(&self) -> bool {
        self.allow_real_fetcher
    }

    /// Returns whether containers may make outbound network requests.
    pub fn allow_network(&self) -> bool {
        self.allow_network
    }
    /// Returns whether workflow writes to the repository are allowed.
    pub fn allow_repo_writes(&self) -> bool {
        self.allow_repo_writes
    }

    /// Returns the stable identity for this run.
    pub fn run_id(&self) -> &str {
        &self.run_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config_starts_with_defaults() {
        let config = ActRunConfig::new();
        assert!(config.workflow().is_none());
        assert!(config.job().is_none());
        assert!(config.event().is_none());
        assert!(config.inputs().is_empty());
        assert!(config.secrets().is_empty());
    }

    #[test]
    fn builder_adds_workflow_job_and_event() {
        let config = ActRunConfig::new()
            .with_workflow(ActWorkflow::new(".github/workflows/ci.yml".into()))
            .with_job(ActJob::new("test".into()))
            .with_event(ActEvent::new("push".into()));

        assert_eq!(
            config.workflow().unwrap().as_str(),
            ".github/workflows/ci.yml"
        );
        assert_eq!(config.job().unwrap().as_str(), "test");
        assert_eq!(config.event().unwrap().as_str(), "push");
    }

    #[test]
    fn builder_adds_inputs() {
        let config =
            ActRunConfig::new().add_input(ActInput::new("environment".into(), "staging".into()));

        assert_eq!(config.inputs()[0].key(), "environment");
        assert_eq!(config.inputs()[0].value(), "staging");
    }

    #[test]
    fn add_secret_keeps_name_and_value() {
        let config = ActRunConfig::new().add_secret(Secret::new("KEY".into(), "value".into()));

        assert_eq!(config.secrets().len(), 1);
        assert_eq!(config.secrets()[0].name(), "KEY");
        assert_eq!(config.secrets()[0].value(), "value");
    }

    #[test]
    fn default_creates_empty_config() {
        let config = ActRunConfig::default();
        assert!(config.workflow.is_none());
        assert!(config.job.is_none());
    }

    #[test]
    fn new_config_disables_all_workflows() {
        assert!(!ActRunConfig::new().all_workflows());
    }

    #[test]
    fn new_config_disables_every_allow_flag() {
        let config = ActRunConfig::new();
        assert!(!config.allow_real_container());
        assert!(!config.allow_real_fetcher());
        assert!(!config.allow_network());
    }

    #[test]
    fn with_allow_real_container_opts_into_the_real_runtime() {
        assert!(
            ActRunConfig::new()
                .with_allow_real_container(true)
                .allow_real_container()
        );
    }

    #[test]
    fn with_allow_real_fetcher_opts_into_the_real_fetcher() {
        assert!(
            ActRunConfig::new()
                .with_allow_real_fetcher(true)
                .allow_real_fetcher()
        );
    }

    #[test]
    fn with_allow_network_permits_outbound_requests() {
        assert!(ActRunConfig::new().with_allow_network(true).allow_network());
    }

    #[test]
    fn with_all_workflows_enables_running_every_workflow() {
        assert!(ActRunConfig::new().with_all_workflows(true).all_workflows());
    }
}
