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
        let Self {
            workflow,
            job,
            event,
            event_path,
            workflow_directory,
            inputs,
            secrets,
            all_workflows,
            allow_repo_writes,
            allow_real_container,
            allow_real_fetcher,
            allow_network,
        } = self;
        let config = Self::apply_named_values(
            WorkflowRunConfig::new(),
            workflow,
            job,
            event,
            workflow_directory,
            event_path,
        )?;
        let config = Self::apply_collection_values(config, inputs, secrets);
        Ok(Self::apply_flags(
            config,
            all_workflows,
            allow_repo_writes,
            allow_real_container,
            allow_real_fetcher,
            allow_network,
        ))
    }

    fn apply_named_values(
        config: WorkflowRunConfig,
        workflow: Option<String>,
        job: Option<String>,
        event: Option<String>,
        workflow_directory: Option<PathBuf>,
        event_path: Option<PathBuf>,
    ) -> Result<WorkflowRunConfig, CoreError> {
        let config = match workflow {
            Some(workflow) => config.with_workflow(WorkflowPath::new(workflow)),
            None => config,
        };
        let config = match job {
            Some(job) => config.with_job(JobName::new(job)),
            None => config,
        };
        let config = match event {
            Some(event) => config.with_event(WorkflowEvent::new(event)),
            None => config,
        };
        let config = match workflow_directory {
            Some(workflow_directory) => config.with_workflow_directory(workflow_directory),
            None => config,
        };
        match event_path {
            Some(event_path) => Ok(config.with_event_path(EventPath::new(event_path)?)),
            None => Ok(config),
        }
    }

    fn apply_collection_values(
        config: WorkflowRunConfig,
        inputs: Vec<(String, String)>,
        secrets: Vec<(String, String)>,
    ) -> WorkflowRunConfig {
        let config = inputs.into_iter().fold(config, |config, (key, value)| {
            config.add_input(WorkflowInput::new(key, value))
        });
        secrets.into_iter().fold(config, |config, (name, value)| {
            config.add_secret(Secret::new(name, value))
        })
    }

    fn apply_flags(
        config: WorkflowRunConfig,
        all_workflows: bool,
        allow_repo_writes: bool,
        allow_real_container: bool,
        allow_real_fetcher: bool,
        allow_network: bool,
    ) -> WorkflowRunConfig {
        config
            .with_all_workflows(all_workflows)
            .with_allow_repo_writes(allow_repo_writes)
            .with_allow_real_container(allow_real_container)
            .with_allow_real_fetcher(allow_real_fetcher)
            .with_allow_network(allow_network)
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
