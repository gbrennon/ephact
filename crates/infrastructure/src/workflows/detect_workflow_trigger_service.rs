use crate::{
    application::ports::outbound::DetectWorkflowTriggerPort, domain::value_objects::TriggerKind,
    workflows::workflow_document::WorkflowDocument,
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
        let kind = match event_name {
            "push" => TriggerKind::Push,
            "pull_request" => TriggerKind::PullRequest,
            "tag" => TriggerKind::Tag,
            "workflow_dispatch" => TriggerKind::Manual,
            "schedule" => TriggerKind::Schedule,
            _ => return false,
        };
        WorkflowDocument::parse(workflow_content)
            .map(|doc| doc.into_domain().triggers_on(kind))
            .unwrap_or(false)
    }
}
