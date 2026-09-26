use std::collections::HashMap;

/// Result of building a job environment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildJobEnvironmentResponse {
    env: HashMap<String, String>,
}

impl BuildJobEnvironmentResponse {
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
