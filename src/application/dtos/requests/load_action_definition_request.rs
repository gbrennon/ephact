use std::path::{Path, PathBuf};

/// Request DTO for the
/// [`LoadActionDefinitionPort`](crate::application::ports::inbound::load_action_definition_port::LoadActionDefinitionPort)
/// inbound port.
pub struct LoadActionDefinitionRequest {
    /// Directory holding the action's `action.yml`.
    action_dir: PathBuf,
}

impl LoadActionDefinitionRequest {
    /// Creates a new request.
    pub fn new(action_dir: PathBuf) -> Self {
        Self { action_dir }
    }

    /// Directory holding the action's `action.yml`.
    pub fn action_dir(&self) -> &Path {
        &self.action_dir
    }
}
