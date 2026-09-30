use std::collections::HashMap;

use serde::Deserialize;

use crate::domain::value_objects::WorkflowTrigger;

/// A scalar-or-sequence YAML field, matching Woodpecker's habit of allowing
/// either `event: push` or `event: [push, pull_request]`.
#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(untagged)]
enum StringOrVec {
    One(String),
    Many(Vec<String>),
}

impl StringOrVec {
    fn into_vec(self) -> Vec<String> {
        match self {
            Self::One(value) => vec![value],
            Self::Many(values) => values,
        }
    }
}

/// A single Woodpecker `when:` condition.
///
/// Only the `event` selector is interpreted; other Woodpecker constraints
/// (branch, path, cron, ...) are intentionally ignored here because they have no
/// counterpart in the shared trigger model.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct WoodpeckerWhenYaml {
    #[serde(default)]
    event: Option<StringOrVec>,
}

impl WoodpeckerWhenYaml {
    /// Maps this condition into the domain triggers it enables.
    #[must_use]
    pub fn into_triggers(self) -> Vec<WorkflowTrigger> {
        self.event
            .map(StringOrVec::into_vec)
            .unwrap_or_default()
            .into_iter()
            .filter_map(Self::trigger_for_event)
            .collect()
    }

    fn trigger_for_event(event: String) -> Option<WorkflowTrigger> {
        match event.as_str() {
            "push" => Some(WorkflowTrigger::Push(None)),
            "pull_request" => Some(WorkflowTrigger::PullRequest(None)),
            "manual" => Some(WorkflowTrigger::Manual {
                inputs: HashMap::new(),
            }),
            _ => None,
        }
    }
}
