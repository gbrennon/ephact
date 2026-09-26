use std::collections::HashMap;

/// Response DTO for the
/// `BuildActionInputEnvironmentPort`
/// outbound port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildActionInputEnvironmentResponse {
    env: HashMap<String, String>,
}

impl BuildActionInputEnvironmentResponse {
    pub fn new(env: HashMap<String, String>) -> Self {
        Self { env }
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn into_env(self) -> HashMap<String, String> {
        self.env
    }
}
