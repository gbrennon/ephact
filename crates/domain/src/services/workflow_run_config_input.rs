#[derive(Default)]
pub struct WorkflowRunConfigInput {
    pub(super) workflow: Option<String>,
    pub(super) job: Option<String>,
    pub(super) event: Option<String>,
    pub(super) inputs: Vec<(String, String)>,
    pub(super) secrets: Vec<(String, String)>,
    pub(super) all_workflows: bool,
    pub(super) allow_repo_writes: bool,
    pub(super) allow_real_container: bool,
    pub(super) allow_real_fetcher: bool,
    pub(super) allow_network: bool,
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
