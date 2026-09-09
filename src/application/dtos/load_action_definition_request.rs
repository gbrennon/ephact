use std::path::Path;

/// Request DTO for the
/// [`LoadActionDefinitionPort`](crate::application::ports::inbound::load_action_definition_port::LoadActionDefinitionPort)
/// inbound port.
pub struct LoadActionDefinitionRequest<'a> {
    /// Directory holding the action's `action.yml`.
    action_dir: &'a Path,
}

impl<'a> LoadActionDefinitionRequest<'a> {
    /// Creates a new request.
    pub fn new(action_dir: &'a Path) -> Self {
        Self { action_dir }
    }

    /// Directory holding the action's `action.yml`.
    pub fn action_dir(&self) -> &'a Path {
        self.action_dir
    }
}
