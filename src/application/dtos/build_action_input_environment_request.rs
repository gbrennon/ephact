use std::collections::HashMap;

/// Request DTO for the
/// `BuildActionInputEnvironmentPort`
/// inbound port.
pub struct BuildActionInputEnvironmentRequest<'a> {
    pub env: &'a HashMap<String, String>,
    pub inputs: &'a HashMap<String, String>,
    pub action_path: &'a str,
}

impl<'a> BuildActionInputEnvironmentRequest<'a> {
    pub fn new(
        env: &'a HashMap<String, String>,
        inputs: &'a HashMap<String, String>,
        action_path: &'a str,
    ) -> Self {
        Self {
            env,
            inputs,
            action_path,
        }
    }

    pub fn env(&self) -> &'a HashMap<String, String> {
        self.env
    }

    pub fn inputs(&self) -> &'a HashMap<String, String> {
        self.inputs
    }

    pub fn action_path(&self) -> &'a str {
        self.action_path
    }
}
