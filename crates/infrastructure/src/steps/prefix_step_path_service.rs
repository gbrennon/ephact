use std::collections::HashMap;

use crate::application::{
    dtos::requests::PrefixStepPathRequest,
    ports::outbound::step_path_prefixer_port::StepPathPrefixerPort,
};

/// Service that prefixes a step's `PATH` with the directories earlier steps
/// exported through `GITHUB_PATH`.
pub struct PrefixStepPathService;

impl PrefixStepPathService {
    pub fn new() -> Self {
        Self
    }
}

impl Default for PrefixStepPathService {
    fn default() -> Self {
        Self::new()
    }
}

impl StepPathPrefixerPort for PrefixStepPathService {
    fn prefix(&self, request: PrefixStepPathRequest) -> HashMap<String, String> {
        let base = request.env().get("PATH").cloned().unwrap_or_default();
        let path = if request.path_additions().is_empty() {
            base
        } else {
            format!("{}:{}", request.path_additions().join(":"), base)
        };

        let mut env = request.env().clone();
        env.insert("PATH".into(), path);
        env
    }
}
