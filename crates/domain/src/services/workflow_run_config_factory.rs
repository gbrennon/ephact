use crate::value_objects::{
    ActEvent, ActInput, ActJob, ActWorkflow, Secret, WorkflowRunConfig,
    workflow_run_config::WorkflowRunConfigParts,
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
}

pub struct WorkflowRunConfigFactory;

impl WorkflowRunConfigFactory {
    pub fn create(input: WorkflowRunConfigInput) -> WorkflowRunConfig {
        WorkflowRunConfig::from_parts(
            WorkflowRunConfigParts::default()
                .with_workflow(input.workflow.map(ActWorkflow::new))
                .with_job(input.job.map(ActJob::new))
                .with_event(input.event.map(ActEvent::new))
                .with_inputs(
                    input
                        .inputs
                        .into_iter()
                        .map(|(key, value)| ActInput::new(key, value))
                        .collect(),
                )
                .with_secrets(
                    input
                        .secrets
                        .into_iter()
                        .map(|(name, value)| Secret::new(name, value))
                        .collect(),
                )
                .with_all_workflows(input.all_workflows)
                .with_allow_repo_writes(input.allow_repo_writes)
                .with_allow_real_container(input.allow_real_container)
                .with_allow_real_fetcher(input.allow_real_fetcher)
                .with_allow_network(input.allow_network),
        )
    }
}
