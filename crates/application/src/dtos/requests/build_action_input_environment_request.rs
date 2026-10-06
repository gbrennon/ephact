use std::collections::HashMap;

/// Inputs for building an action environment from environment entries, action inputs, and an action path.
pub struct BuildActionInputEnvironmentRequest {
    env: HashMap<String, String>,
    inputs: HashMap<String, String>,
    action_path: String,
}

impl BuildActionInputEnvironmentRequest {
    /// Creates inputs from environment entries, action inputs, and an action path.
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

    /// Returns the environment entries.
    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    /// Returns the action inputs.
    pub fn inputs(&self) -> &HashMap<String, String> {
        &self.inputs
    }

    /// Returns the action path.
    pub fn action_path(&self) -> &str {
        &self.action_path
    }
}
