use std::collections::HashMap;

use crate::domain::value_objects::{TriggerFilter, WorkflowDispatchInput};

/// The event(s) that trigger a workflow.
///
/// GitHub Actions supports three forms for the `on` field:
/// - **Scalar**: `on: push`
/// - **Sequence**: `on: [push, pull_request]`
/// - **Mapping**: `on: { push: { branches: [main] } }`
///
/// Each form is a distinct variant, so a caller can ask which spelling a
/// workflow used as well as which events it declares.
///
/// # Examples
///
/// ```
/// use ephact::domain::value_objects::WorkflowTrigger;
///
/// let single = WorkflowTrigger::Single("push".to_owned());
/// assert!(single.is_single("push"));
///
/// let multiple =
///     WorkflowTrigger::Multiple(vec!["push".to_owned(), "pull_request".to_owned()]);
/// assert!(multiple.is_multiple());
/// assert!(multiple.has_event("pull_request"));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum WorkflowTrigger {
    /// A single event name (e.g. `on: push`).
    Single(String),
    /// Multiple event names (e.g. `on: [push, pull_request]`).
    Multiple(Vec<String>),
    /// Event names with type-specific configuration
    /// (e.g. `on: { push: { branches: [main] } }`).
    WithTypes(HashMap<String, Option<TriggerFilter>>),
}

impl WorkflowTrigger {
    /// Returns `true` if the `on` field matches a single event name.
    pub fn is_single(&self, name: &str) -> bool {
        matches!(self, WorkflowTrigger::Single(n) if n == name)
    }

    /// Returns `true` if the `on` field contains multiple events.
    pub fn is_multiple(&self) -> bool {
        matches!(self, WorkflowTrigger::Multiple(_))
    }

    /// Returns `true` if the `on` field has type-specific configuration.
    pub fn has_types(&self) -> bool {
        matches!(self, WorkflowTrigger::WithTypes(_))
    }

    /// Returns `true` if the given event name is present in any form.
    pub fn has_event(&self, name: &str) -> bool {
        match self {
            WorkflowTrigger::Single(n) => n == name,
            WorkflowTrigger::Multiple(names) => names.iter().any(|n| n == name),
            WorkflowTrigger::WithTypes(map) => map.contains_key(name),
        }
    }

    /// Returns all event names regardless of form.
    pub fn event_names(&self) -> Vec<&str> {
        match self {
            WorkflowTrigger::Single(name) => vec![name.as_str()],
            WorkflowTrigger::Multiple(names) => names.iter().map(|s| s.as_str()).collect(),
            WorkflowTrigger::WithTypes(map) => map.keys().map(|s| s.as_str()).collect(),
        }
    }
    pub fn workflow_dispatch_inputs(&self) -> Option<&HashMap<String, WorkflowDispatchInput>> {
        match self {
            Self::WithTypes(events) => events
                .get("workflow_dispatch")
                .and_then(Option::as_ref)
                .map(TriggerFilter::inputs),
            _ => None,
        }
    }
}

impl Default for WorkflowTrigger {
    fn default() -> Self {
        WorkflowTrigger::Single("push".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn events(names: &[&str]) -> WorkflowTrigger {
        WorkflowTrigger::WithTypes(
            names
                .iter()
                .map(|name| ((*name).to_owned(), None))
                .collect(),
        )
    }

    #[test]
    fn single_reports_only_its_own_event() {
        let trigger = WorkflowTrigger::Single("push".to_owned());

        assert!(trigger.is_single("push"));
        assert!(!trigger.is_single("pull_request"));
        assert!(trigger.has_event("push"));
        assert!(!trigger.has_event("pull_request"));
        assert_eq!(trigger.event_names(), vec!["push"]);
    }

    #[test]
    fn multiple_reports_every_listed_event() {
        let trigger = WorkflowTrigger::Multiple(vec!["push".to_owned(), "pull_request".to_owned()]);

        assert!(trigger.is_multiple());
        assert!(trigger.has_event("push"));
        assert!(trigger.has_event("pull_request"));
        assert_eq!(trigger.event_names(), vec!["push", "pull_request"]);
    }

    #[test]
    fn with_types_reports_every_configured_event() {
        let trigger = events(&["push", "pull_request"]);

        assert!(trigger.has_types());
        assert!(trigger.has_event("push"));
        assert!(trigger.has_event("pull_request"));
        assert!(!trigger.has_event("schedule"));

        let mut names = trigger.event_names();
        names.sort_unstable();
        assert_eq!(names, vec!["pull_request", "push"]);
    }

    #[test]
    fn workflow_dispatch_inputs_are_exposed_when_configured() {
        let inputs = HashMap::from([(
            "name".to_owned(),
            WorkflowDispatchInput::new(None, true, None, Some("string".to_owned()), Vec::new()),
        )]);
        let filter = TriggerFilter::new(
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            inputs.clone(),
            Vec::new(),
        );
        let trigger = WorkflowTrigger::WithTypes(HashMap::from([(
            "workflow_dispatch".to_owned(),
            Some(filter),
        )]));

        assert_eq!(trigger.workflow_dispatch_inputs(), Some(&inputs));
    }

    #[test]
    fn workflow_dispatch_inputs_are_absent_without_the_event() {
        assert_eq!(
            WorkflowTrigger::Single("push".to_owned()).workflow_dispatch_inputs(),
            None
        );
        assert_eq!(
            events(&["workflow_dispatch"]).workflow_dispatch_inputs(),
            None
        );
    }

    #[test]
    fn default_is_single_push() {
        assert_eq!(
            WorkflowTrigger::default(),
            WorkflowTrigger::Single("push".to_owned())
        );
    }
}
