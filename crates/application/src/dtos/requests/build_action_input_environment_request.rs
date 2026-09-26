use std::collections::HashMap;

/// Request DTO for the
/// `BuildActionInputEnvironmentPort`
/// inbound port.
pub struct BuildActionInputEnvironmentRequest {
    env: HashMap<String, String>,
    inputs: HashMap<String, String>,
    action_path: String,
}

impl BuildActionInputEnvironmentRequest {
    pub fn new(
        env: HashMap<String, String>,
        inputs: HashMap<String, String>,
        action_path: String,
    ) -> Self {
        Self {
            env,
            inputs,
            action_path,
        }
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn inputs(&self) -> &HashMap<String, String> {
        &self.inputs
    }

    pub fn action_path(&self) -> &str {
        &self.action_path
    }
}
