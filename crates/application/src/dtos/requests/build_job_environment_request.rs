use std::collections::HashMap;

use crate::domain::{aggregates::Workflow, value_objects::EvaluationContext};

/// Inputs for building a job environment from a workflow and environment entries.
pub struct BuildJobEnvironmentRequest {
    workflow: Workflow,
    job_env: HashMap<String, String>,
    context: EvaluationContext,
    event_payload: Option<String>,
}

impl BuildJobEnvironmentRequest {
    /// Creates inputs from a workflow and environment entries.
    pub fn new(workflow: Workflow, job_env: HashMap<String, String>) -> Self {
        Self {
            workflow,
            job_env,
            context: EvaluationContext::new(),
            event_payload: None,
        }
    }

    /// Returns the workflow.
    pub fn workflow(&self) -> &Workflow {
        &self.workflow
    }

    /// Returns the environment entries.
    pub fn job_env(&self) -> &HashMap<String, String> {
        &self.job_env
    }
    /// Adds the evaluated run context used by runner-specific adapters.
    pub fn with_context(mut self, context: EvaluationContext) -> Self {
        self.context = context;
        self
    }

    /// Returns the evaluated run context.
    pub fn context(&self) -> &EvaluationContext {
        &self.context
    }
    /// Carries the raw event payload to runner-specific adapters.
    pub fn with_event_payload_opt(mut self, event_payload: Option<&str>) -> Self {
        self.event_payload = event_payload.map(str::to_owned);
        self
    }

    /// Returns the raw event payload carried to runner-specific adapters.
    pub fn event_payload(&self) -> Option<&str> {
        self.event_payload.as_deref()
    }
}
