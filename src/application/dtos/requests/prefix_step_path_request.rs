use std::collections::HashMap;

/// Request DTO for the
/// [`PrefixStepPathPort`](crate::application::ports::inbound::prefix_step_path_port::PrefixStepPathPort)
/// inbound port.
pub struct PrefixStepPathRequest {
    /// Environment whose `PATH` is prefixed.
    env: HashMap<String, String>,
    /// Directories earlier steps exported through `GITHUB_PATH`.
    path_additions: Vec<String>,
}

impl PrefixStepPathRequest {
    /// Creates a new request.
    pub fn new(env: HashMap<String, String>, path_additions: Vec<String>) -> Self {
        Self {
            env,
            path_additions,
        }
    }

    /// Environment whose `PATH` is prefixed.
    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    /// Directories earlier steps exported through `GITHUB_PATH`.
    pub fn path_additions(&self) -> &[String] {
        &self.path_additions
    }
}
