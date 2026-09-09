use std::collections::HashMap;

/// Request DTO for the
/// [`PrefixStepPathPort`](crate::application::ports::inbound::prefix_step_path_port::PrefixStepPathPort)
/// inbound port.
pub struct PrefixStepPathRequest<'a> {
    /// Environment whose `PATH` is prefixed.
    env: &'a HashMap<String, String>,
    /// Directories earlier steps exported through `GITHUB_PATH`.
    path_additions: &'a [String],
}

impl<'a> PrefixStepPathRequest<'a> {
    /// Creates a new request.
    pub fn new(env: &'a HashMap<String, String>, path_additions: &'a [String]) -> Self {
        Self {
            env,
            path_additions,
        }
    }

    /// Environment whose `PATH` is prefixed.
    pub fn env(&self) -> &'a HashMap<String, String> {
        self.env
    }

    /// Directories earlier steps exported through `GITHUB_PATH`.
    pub fn path_additions(&self) -> &'a [String] {
        self.path_additions
    }
}
