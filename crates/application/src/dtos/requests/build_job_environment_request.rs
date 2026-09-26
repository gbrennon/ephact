use std::collections::HashMap;

use crate::domain::aggregates::Workflow;

/// Request data for building a job environment.
pub struct BuildJobEnvironmentRequest {
    workflow: Workflow,
    job_env: HashMap<String, String>,
}

impl BuildJobEnvironmentRequest {
    pub fn new(workflow: Workflow, job_env: HashMap<String, String>) -> Self {
        Self { workflow, job_env }
    }

    pub fn workflow(&self) -> &Workflow {
        &self.workflow
    }

    pub fn job_env(&self) -> &HashMap<String, String> {
        &self.job_env
    }
}
