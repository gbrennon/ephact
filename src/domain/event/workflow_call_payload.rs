use std::collections::HashMap;

use serde::Serialize;

/// Payload for `workflow_call` events.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct WorkflowCallPayload {
    inputs: HashMap<String, serde_json::Value>,
    secrets: HashMap<String, String>,
}

impl WorkflowCallPayload {
    pub fn new(inputs: HashMap<String, serde_json::Value>, secrets: HashMap<String, String>) -> Self {
        Self { inputs, secrets }
    }

    pub fn inputs(&self) -> &HashMap<String, serde_json::Value> {
        &self.inputs
    }

    pub fn secrets(&self) -> &HashMap<String, String> {
        &self.secrets
    }
}
