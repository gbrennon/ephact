use std::collections::HashMap;

use crate::domain::workflow::Workflow;

/// Request DTO for the
/// [`BuildJobEnvironmentPort`](crate::application::ports::outbound::build_job_environment_port::BuildJobEnvironmentPort)
/// outbound port.
pub struct BuildJobEnvironmentRequest<'a> {
    workflow: &'a Workflow,
    job_env: &'a HashMap<String, String>,
}

impl<'a> BuildJobEnvironmentRequest<'a> {
    pub fn new(workflow: &'a Workflow, job_env: &'a HashMap<String, String>) -> Self {
        Self { workflow, job_env }
    }

    pub fn workflow(&self) -> &'a Workflow {
        self.workflow
    }

    pub fn job_env(&self) -> &'a HashMap<String, String> {
        self.job_env
    }
}
