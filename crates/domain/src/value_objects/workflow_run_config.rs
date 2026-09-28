use crate::value_objects::{JobName, Secret, WorkflowEvent, WorkflowInput, WorkflowPath};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkflowRunConfig {
    workflow: Option<WorkflowPath>,
    job: Option<JobName>,
    event: Option<WorkflowEvent>,
    inputs: Vec<WorkflowInput>,
    secrets: Vec<Secret>,
    all_workflows: bool,
    allow_repo_writes: bool,
    allow_real_container: bool,
    allow_real_fetcher: bool,
    allow_network: bool,
}

/// Constructors for [`WorkflowRunConfig`].
impl WorkflowRunConfig {
    /// Creates a new config with sensible defaults.
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
        }
    }
}
impl Default for WorkflowRunConfig {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder API - fluent setters that consume and return `Self`.
impl WorkflowRunConfig {
    /// Sets the workflow file to run.
    pub fn with_workflow(mut self, workflow: WorkflowPath) -> Self {
        self.workflow = Some(workflow);
        self
    }

    /// Sets the specific job to run within the workflow.
    pub fn with_job(mut self, job: JobName) -> Self {
        self.job = Some(job);
        self
    }

    /// Sets the event to simulate.
    pub fn with_event(mut self, event: WorkflowEvent) -> Self {
        self.event = Some(event);
        self
    }

    /// Adds an input variable.
    pub fn add_input(mut self, input: WorkflowInput) -> Self {
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

/// Read-only access to each field of [`WorkflowRunConfig`].
impl WorkflowRunConfig {
    /// Returns the workflow, if set.
    pub fn workflow(&self) -> Option<&WorkflowPath> {
        self.workflow.as_ref()
    }

    /// Returns the job, if set.
    pub fn job(&self) -> Option<&JobName> {
        self.job.as_ref()
    }

    /// Returns the event, if set.
    pub fn event(&self) -> Option<&WorkflowEvent> {
        self.event.as_ref()
    }

    /// Returns whether the configuration contains the event required to run a workflow.
    pub fn is_valid(&self) -> bool {
        self.event.is_some()
    }

    /// Returns all input variables.
    pub fn inputs(&self) -> &[WorkflowInput] {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config_starts_with_defaults() {
        let config = WorkflowRunConfig::new();
        assert!(config.workflow().is_none());
        assert!(config.job().is_none());
        assert!(config.event().is_none());
        assert!(config.inputs().is_empty());
        assert!(config.secrets().is_empty());
    }

    #[test]
    fn config_without_event_is_invalid() {
        assert!(!WorkflowRunConfig::new().is_valid());
    }

    #[test]
    fn config_with_event_is_valid() {
        let config = WorkflowRunConfig::new().with_event(WorkflowEvent::new("push".into()));

        assert!(config.is_valid());
    }

    #[test]
    fn builder_adds_workflow_job_and_event() {
        let config = WorkflowRunConfig::new()
            .with_workflow(WorkflowPath::new(".github/workflows/ci.yml".into()))
            .with_job(JobName::new("test".into()))
            .with_event(WorkflowEvent::new("push".into()));

        assert_eq!(
            config.workflow().unwrap().as_str(),
            ".github/workflows/ci.yml"
        );
        assert_eq!(config.job().unwrap().as_str(), "test");
        assert_eq!(config.event().unwrap().as_str(), "push");
    }

    #[test]
    fn builder_adds_inputs() {
        let config = WorkflowRunConfig::new()
            .add_input(WorkflowInput::new("environment".into(), "staging".into()));

        assert_eq!(config.inputs()[0].key(), "environment");
        assert_eq!(config.inputs()[0].value(), "staging");
    }

    #[test]
    fn add_secret_keeps_name_and_value() {
        let config = WorkflowRunConfig::new().add_secret(Secret::new("KEY".into(), "value".into()));

        assert_eq!(config.secrets().len(), 1);
        assert_eq!(config.secrets()[0].name(), "KEY");
        assert_eq!(config.secrets()[0].value(), "value");
    }

    #[test]
    fn default_creates_empty_config() {
        let config = WorkflowRunConfig::new();
        assert!(config.workflow.is_none());
        assert!(config.job.is_none());
    }

    #[test]
    fn new_config_disables_all_workflows() {
        assert!(!WorkflowRunConfig::new().all_workflows());
    }

    #[test]
    fn new_config_disables_every_allow_flag() {
        let config = WorkflowRunConfig::new();
        assert!(!config.allow_real_container());
        assert!(!config.allow_real_fetcher());
        assert!(!config.allow_network());
    }

    #[test]
    fn with_allow_real_container_opts_into_the_real_runtime() {
        assert!(
            WorkflowRunConfig::new()
                .with_allow_real_container(true)
                .allow_real_container()
        );
    }

    #[test]
    fn with_allow_real_fetcher_opts_into_the_real_fetcher() {
        assert!(
            WorkflowRunConfig::new()
                .with_allow_real_fetcher(true)
                .allow_real_fetcher()
        );
    }

    #[test]
    fn with_allow_network_permits_outbound_requests() {
        assert!(
            WorkflowRunConfig::new()
                .with_allow_network(true)
                .allow_network()
        );
    }

    #[test]
    fn with_all_workflows_enables_running_every_workflow() {
        assert!(
            WorkflowRunConfig::new()
                .with_all_workflows(true)
                .all_workflows()
        );
    }
}
