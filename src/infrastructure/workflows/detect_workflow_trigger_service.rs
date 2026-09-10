use crate::{
    application::ports::outbound::DetectWorkflowTriggerPort,
    infrastructure::workflows::yaml::WorkflowYaml,
};

/// Detects a workflow's declared events by parsing its YAML content.
pub struct DetectWorkflowTriggerService;

impl DetectWorkflowTriggerService {
    /// Creates a detector for YAML workflow content.
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for DetectWorkflowTriggerService {
    fn default() -> Self {
        Self::new()
    }
}

impl DetectWorkflowTriggerPort for DetectWorkflowTriggerService {
    fn triggers_on_event(&self, workflow_content: &str, event_name: &str) -> bool {
        serde_yaml::from_str::<WorkflowYaml>(workflow_content)
            .map(|parsed| parsed.into_domain().trigger().has_event(event_name))
            .unwrap_or(false)
    }
}
