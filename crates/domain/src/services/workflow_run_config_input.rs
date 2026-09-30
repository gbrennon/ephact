use crate::value_objects::{
    JobName, Secret, WorkflowEvent, WorkflowInput, WorkflowPath, WorkflowRunConfig,
};

#[derive(Default)]
pub struct WorkflowRunConfigInput {
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
}

impl WorkflowRunConfigInput {
    pub fn with_workflow(mut self, workflow: Option<String>) -> Self {
        self.workflow = workflow;
        self
    }

    pub fn with_job(mut self, job: Option<String>) -> Self {
        self.job = job;
        self
    }

    pub fn with_event(mut self, event: Option<String>) -> Self {
        self.event = event;
        self
    }

    pub fn with_inputs(mut self, inputs: Vec<(String, String)>) -> Self {
        self.inputs = inputs;
        self
    }

    pub fn with_secrets(mut self, secrets: Vec<(String, String)>) -> Self {
        self.secrets = secrets;
        self
    }

    pub fn with_all_workflows(mut self, all_workflows: bool) -> Self {
        self.all_workflows = all_workflows;
        self
    }

    pub fn with_allow_repo_writes(mut self, allow_repo_writes: bool) -> Self {
        self.allow_repo_writes = allow_repo_writes;
        self
    }

    pub fn with_allow_real_container(mut self, allow_real_container: bool) -> Self {
        self.allow_real_container = allow_real_container;
        self
    }

    pub fn with_allow_real_fetcher(mut self, allow_real_fetcher: bool) -> Self {
        self.allow_real_fetcher = allow_real_fetcher;
        self
    }

    pub fn with_allow_network(mut self, allow_network: bool) -> Self {
        self.allow_network = allow_network;
        self
    }

    /// Consumes the input and builds its domain workflow configuration.
    pub fn into_config(self) -> WorkflowRunConfig {
        let mut config = WorkflowRunConfig::new();

        if let Some(workflow) = self.workflow {
            config = config.with_workflow(WorkflowPath::new(workflow));
        }
        if let Some(job) = self.job {
            config = config.with_job(JobName::new(job));
        }
        if let Some(event) = self.event {
            config = config.with_event(WorkflowEvent::new(event));
        }
        for (key, value) in self.inputs {
            config = config.add_input(WorkflowInput::new(key, value));
        }
        for (name, value) in self.secrets {
            config = config.add_secret(Secret::new(name, value));
        }

        config
            .with_all_workflows(self.all_workflows)
            .with_allow_repo_writes(self.allow_repo_writes)
            .with_allow_real_container(self.allow_real_container)
            .with_allow_real_fetcher(self.allow_real_fetcher)
            .with_allow_network(self.allow_network)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_builds_default_config() {
        let config = WorkflowRunConfigInput::default().into_config();

        assert!(config.workflow().is_none());
        assert!(config.job().is_none());
        assert!(config.event().is_none());
        assert!(config.inputs().is_empty());
        assert!(config.secrets().is_empty());
        assert!(!config.all_workflows());
        assert!(!config.allow_repo_writes());
        assert!(!config.allow_real_container());
        assert!(!config.allow_real_fetcher());
        assert!(!config.allow_network());
    }

    #[test]
    fn fully_populated_input_builds_matching_config() {
        let config = WorkflowRunConfigInput::default()
            .with_workflow(Some(".github/workflows/ci.yml".into()))
            .with_job(Some("test".into()))
            .with_event(Some("push".into()))
            .with_inputs(vec![("environment".into(), "staging".into())])
            .with_secrets(vec![("TOKEN".into(), "value".into())])
            .with_all_workflows(true)
            .with_allow_repo_writes(true)
            .with_allow_real_container(true)
            .with_allow_real_fetcher(true)
            .with_allow_network(true)
            .into_config();

        assert_eq!(
            config.workflow().unwrap().as_str(),
            ".github/workflows/ci.yml"
        );
        assert_eq!(config.job().unwrap().as_str(), "test");
        assert_eq!(config.event().unwrap().as_str(), "push");
        assert_eq!(config.inputs().len(), 1);
        assert_eq!(config.secrets().len(), 1);
        assert!(config.all_workflows());
        assert!(config.allow_repo_writes());
        assert!(config.allow_real_container());
        assert!(config.allow_real_fetcher());
        assert!(config.allow_network());
    }
}
