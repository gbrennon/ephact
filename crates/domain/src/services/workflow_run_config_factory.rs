use crate::{
    services::workflow_run_config_input::WorkflowRunConfigInput,
    value_objects::{
        JobName, Secret, WorkflowEvent, WorkflowInput, WorkflowPath, WorkflowRunConfig,
    },
};

pub struct WorkflowRunConfigFactory;

impl WorkflowRunConfigFactory {
    pub fn create(input: WorkflowRunConfigInput) -> WorkflowRunConfig {
        let mut config = WorkflowRunConfig::new();

        if let Some(workflow) = input.workflow {
            config = config.with_workflow(WorkflowPath::new(workflow));
        }
        if let Some(job) = input.job {
            config = config.with_job(JobName::new(job));
        }
        if let Some(event) = input.event {
            config = config.with_event(WorkflowEvent::new(event));
        }
        for (key, value) in input.inputs {
            config = config.add_input(WorkflowInput::new(key, value));
        }
        for (name, value) in input.secrets {
            config = config.add_secret(Secret::new(name, value));
        }

        config
            .with_all_workflows(input.all_workflows)
            .with_allow_repo_writes(input.allow_repo_writes)
            .with_allow_real_container(input.allow_real_container)
            .with_allow_real_fetcher(input.allow_real_fetcher)
            .with_allow_network(input.allow_network)
    }
}
