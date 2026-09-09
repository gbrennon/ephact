use std::collections::HashMap;

use serde::Serialize;

/// Payload for `workflow_call` events.
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct WorkflowCallPayload {
    inputs: HashMap<String, serde_json::Value>,
    secrets: HashMap<String, String>,
}

impl WorkflowCallPayload {
    pub fn new(
        inputs: HashMap<String, serde_json::Value>,
        secrets: HashMap<String, String>,
    ) -> Self {
        Self { inputs, secrets }
    }

    pub fn inputs(&self) -> &HashMap<String, serde_json::Value> {
        &self.inputs
    }

    pub fn secrets(&self) -> &HashMap<String, String> {
        &self.secrets
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_preserves_fields() {
        let payload = WorkflowCallPayload::new(
            HashMap::from([("input".into(), serde_json::json!("value"))]),
            HashMap::from([("secret".into(), "value".into())]),
        );

        assert_eq!(payload.inputs()["input"], serde_json::json!("value"));
        assert_eq!(payload.secrets()["secret"], "value");
    }
}
