use std::collections::HashMap;

use crate::value_objects::ContextValue;

#[derive(Debug, Clone, PartialEq, Default)]
pub(super) struct JobContract {
    /// Outputs produced by this job (for dependent jobs).
    outputs: HashMap<String, String>,

    /// Input parameters passed via `workflow_call`.
    with: Option<ContextValue>,

    /// Secrets available to this job.
    secrets: Option<ContextValue>,
}

impl JobContract {
    pub(super) fn with_outputs(mut self, outputs: HashMap<String, String>) -> Self {
        self.outputs = outputs;
        self
    }

    pub(super) fn with_inputs(mut self, inputs: Option<ContextValue>) -> Self {
        self.with = inputs;
        self
    }

    pub(super) fn with_secrets(mut self, secrets: Option<ContextValue>) -> Self {
        self.secrets = secrets;
        self
    }

    pub(super) fn outputs(&self) -> &HashMap<String, String> {
        &self.outputs
    }

    pub(super) fn with(&self) -> Option<&ContextValue> {
        self.with.as_ref()
    }

    pub(super) fn secrets(&self) -> Option<&ContextValue> {
        self.secrets.as_ref()
    }
}
