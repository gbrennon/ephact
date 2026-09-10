use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

use crate::{
    domain::value_objects::WorkflowTrigger,
    infrastructure::workflows::yaml::{TriggerFilterYaml, WorkflowTriggerVisitor},
};

/// The `on:` entry of a workflow file, in any of its three YAML spellings.
#[derive(Debug, Clone, PartialEq)]
pub enum WorkflowTriggerYaml {
    /// A single event name, as in `on: push`.
    Single(String),
    /// A sequence of event names, as in `on: [push, pull_request]`.
    Multiple(Vec<String>),
    /// A mapping of event names to their optional filters.
    WithTypes(HashMap<String, Option<TriggerFilterYaml>>),
}

impl Default for WorkflowTriggerYaml {
    fn default() -> Self {
        Self::Single("push".to_owned())
    }
}

impl WorkflowTriggerYaml {
    /// Builds the domain trigger this YAML describes.
    #[must_use]
    pub fn into_domain(self) -> WorkflowTrigger {
        match self {
            Self::Single(event) => WorkflowTrigger::Single(event),
            Self::Multiple(events) => WorkflowTrigger::Multiple(events),
            Self::WithTypes(events) => WorkflowTrigger::WithTypes(
                events
                    .into_iter()
                    .map(|(event, filter)| (event, filter.map(TriggerFilterYaml::into_domain)))
                    .collect(),
            ),
        }
    }
}

impl<'de> Deserialize<'de> for WorkflowTriggerYaml {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(WorkflowTriggerVisitor)
    }
}
