use std::collections::HashMap;

use crate::application::dtos::responses::RunnerContextResponse;

/// Merges a container's declared environment into a base runner context.
pub struct RunnerContextEnv {
    base: RunnerContextResponse,
}

impl RunnerContextEnv {
    pub fn new(base: RunnerContextResponse) -> Self {
        Self { base }
    }

    /// Returns a runner context extended with the parsed container environment.
    pub fn with_container_env(&self, container_env: Vec<String>) -> RunnerContextResponse {
        self.base
            .clone()
            .with_env_extension(self.parse_env_entries(container_env))
    }

    fn parse_env_entries(&self, entries: Vec<String>) -> HashMap<String, String> {
        entries
            .iter()
            .filter_map(|entry| {
                let mut parts = entry.splitn(2, '=');
                Some((
                    parts.next()?.to_string(),
                    parts.next().unwrap_or("").to_string(),
                ))
            })
            .collect()
    }
}
