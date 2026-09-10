#![allow(dead_code)]
use ephact::application::ports::outbound::DetectWorkflowTriggerPort;

/// Detector whose answer is fixed when the instance is built.
///
/// `always_triggering` and `never_triggering` answer the same way for every
/// content; `only_for_content_containing` answers `true` only for content
/// carrying the given marker, which lets a test observe filtering.
pub struct FakeDetectWorkflowTriggerPort {
    triggering_marker: Option<String>,
    triggers_without_marker: bool,
}

impl FakeDetectWorkflowTriggerPort {
    pub fn always_triggering() -> Self {
        Self {
            triggering_marker: None,
            triggers_without_marker: true,
        }
    }

    pub fn never_triggering() -> Self {
        Self {
            triggering_marker: None,
            triggers_without_marker: false,
        }
    }

    pub fn only_for_content_containing(marker: &str) -> Self {
        Self {
            triggering_marker: Some(marker.to_owned()),
            triggers_without_marker: false,
        }
    }
}

impl DetectWorkflowTriggerPort for FakeDetectWorkflowTriggerPort {
    fn triggers_on_event(&self, workflow_content: &str, _event_name: &str) -> bool {
        match &self.triggering_marker {
            Some(marker) => workflow_content.contains(marker.as_str()),
            None => self.triggers_without_marker,
        }
    }
}
