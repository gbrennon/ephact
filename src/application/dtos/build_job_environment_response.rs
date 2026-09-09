use std::collections::HashMap;

/// Response DTO for the
/// [`BuildJobEnvironmentPort`](crate::application::ports::outbound::build_job_environment_port::BuildJobEnvironmentPort)
/// outbound port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildJobEnvironmentResponse {
    pub env: HashMap<String, String>,
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
