use std::collections::HashMap;

use crate::domain::aggregates::Workflow;

/// Inputs for building a job environment from a workflow and environment entries.
pub struct BuildJobEnvironmentRequest {
    workflow: Workflow,
    job_env: HashMap<String, String>,
}

impl BuildJobEnvironmentRequest {
    /// Creates inputs from a workflow and environment entries.
    pub fn new(workflow: Workflow, job_env: HashMap<String, String>) -> Self {
        Self { workflow, job_env }
    }

    /// Returns the workflow.
    pub fn workflow(&self) -> &Workflow {
        &self.workflow
    }

    /// Returns the environment entries.
    pub fn job_env(&self) -> &HashMap<String, String> {
        &self.job_env
    }
}
