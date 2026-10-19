use std::path::PathBuf;

use crate::domain::{
    errors::CoreError,
    value_objects::{
        EventPath, JobName, Secret, WorkflowEvent, WorkflowInput, WorkflowPath, WorkflowRunConfig,
    },
};

#[derive(Default)]
pub struct WorkflowRunConfigMapper {
    workflow: Option<String>,
    job: Option<String>,
    event: Option<String>,
    event_path: Option<PathBuf>,
    workflow_directory: Option<PathBuf>,
    inputs: Vec<(String, String)>,
    secrets: Vec<(String, String)>,
    all_workflows: bool,
    allow_repo_writes: bool,
    allow_real_container: bool,
    allow_real_fetcher: bool,
    allow_network: bool,
}

impl WorkflowRunConfigMapper {
    pub fn with_workflow(self, workflow: Option<String>) -> Self {
        Self { workflow, ..self }
    }

    pub fn with_job(self, job: Option<String>) -> Self {
        Self { job, ..self }
    }

    pub fn with_event(self, event: Option<String>) -> Self {
        Self { event, ..self }
    }
    pub fn with_event_path(self, event_path: Option<PathBuf>) -> Self {
        Self { event_path, ..self }
    }

    pub fn with_workflow_directory(self, workflow_directory: Option<PathBuf>) -> Self {
        Self {
            workflow_directory,
            ..self
        }
    }

    pub fn with_inputs(self, inputs: Vec<(String, String)>) -> Self {
        Self { inputs, ..self }
    }

    pub fn with_secrets(self, secrets: Vec<(String, String)>) -> Self {
        Self { secrets, ..self }
    }

    pub fn with_all_workflows(self, all_workflows: bool) -> Self {
        Self {
            all_workflows,
            ..self
        }
    }

    pub fn with_allow_repo_writes(self, allow_repo_writes: bool) -> Self {
        Self {
            allow_repo_writes,
            ..self
        }
    }

    pub fn with_allow_real_container(self, allow_real_container: bool) -> Self {
        Self {
            allow_real_container,
            ..self
        }
    }

    pub fn with_allow_real_fetcher(self, allow_real_fetcher: bool) -> Self {
        Self {
            allow_real_fetcher,
            ..self
        }
    }

    pub fn with_allow_network(self, allow_network: bool) -> Self {
        Self {
            allow_network,
            ..self
        }
    }

    /// Consumes the mapper and builds the domain workflow configuration.
    pub fn into_config(self) -> Result<WorkflowRunConfig, CoreError> {
        let mut mapper = self;
        let config = mapper.apply_named_values(WorkflowRunConfig::new())?;
        let config = mapper.apply_collection_values(config);
        Ok(mapper.apply_flags(config))
    }

    fn apply_named_values(
        &mut self,
        mut config: WorkflowRunConfig,
    ) -> Result<WorkflowRunConfig, CoreError> {
        if let Some(workflow) = self.workflow.take() {
            config = config.with_workflow(WorkflowPath::new(workflow));
        }
        if let Some(job) = self.job.take() {
            config = config.with_job(JobName::new(job));
        }
        if let Some(event) = self.event.take() {
            config = config.with_event(WorkflowEvent::new(event));
        }
        if let Some(workflow_directory) = self.workflow_directory.take() {
            config = config.with_workflow_directory(workflow_directory);
        }
        self.apply_event_path(config)
    }
    fn apply_event_path(
        &mut self,
        mut config: WorkflowRunConfig,
    ) -> Result<WorkflowRunConfig, CoreError> {
        if let Some(event_path) = self.event_path.take() {
            config = config.with_event_path(EventPath::new(event_path)?);
        }
        Ok(config)
    }

    fn apply_collection_values(&mut self, mut config: WorkflowRunConfig) -> WorkflowRunConfig {
        for (key, value) in std::mem::take(&mut self.inputs) {
            config = config.add_input(WorkflowInput::new(key, value));
        }
        for (name, value) in std::mem::take(&mut self.secrets) {
            config = config.add_secret(Secret::new(name, value));
        }
        config
    }

    fn apply_flags(&self, config: WorkflowRunConfig) -> WorkflowRunConfig {
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
    fn into_config_builds_default_config() {
        let config = WorkflowRunConfigMapper::default().into_config().unwrap();

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
    fn into_config_builds_populated_config() {
        let config = WorkflowRunConfigMapper::default()
            .with_workflow(Some(".ci/workflows/ci.yml".into()))
            .with_job(Some("test".into()))
            .with_event(Some("push".into()))
            .with_inputs(vec![("environment".into(), "staging".into())])
            .with_secrets(vec![("TOKEN".into(), "value".into())])
            .with_all_workflows(true)
            .with_allow_repo_writes(true)
            .with_allow_real_container(true)
            .with_allow_real_fetcher(true)
            .with_allow_network(true)
            .into_config()
            .unwrap();

        assert_eq!(config.workflow().unwrap().as_str(), ".ci/workflows/ci.yml");
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
