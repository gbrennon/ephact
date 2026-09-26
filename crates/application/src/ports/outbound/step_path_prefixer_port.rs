use std::collections::HashMap;

use crate::dtos::requests::PrefixStepPathRequest;

/// Prefixes an environment's `PATH` with exported directories.
pub trait StepPathPrefixerPort: Send + Sync {
    /// Returns the environment with its `PATH` prefixed by the additions.
    fn prefix(&self, request: PrefixStepPathRequest) -> HashMap<String, String>;
}
