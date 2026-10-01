use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

use super::{TriggerFilterYaml, WorkflowTriggerVisitor};
use crate::domain::value_objects::WorkflowTrigger;

#[derive(Debug, Clone, PartialEq)]
pub enum WorkflowTriggerYaml {
    Single(String),
    Multiple(Vec<String>),
    WithTypes(HashMap<String, Option<TriggerFilterYaml>>),
}

impl Default for WorkflowTriggerYaml {
    fn default() -> Self {
        Self::Single("push".to_owned())
    }
}

impl WorkflowTriggerYaml {
    pub fn into_domain(self) -> Vec<WorkflowTrigger> {
        match self {
            Self::Single(event) => Self::trigger_for_event(event, None),
            Self::Multiple(events) => events
                .into_iter()
                .flat_map(|event| Self::trigger_for_event(event, None))
                .collect(),
            Self::WithTypes(events) => events
                .into_iter()
                .flat_map(|(event, filter)| Self::trigger_for_event(event, filter))
                .collect(),
        }
    }

    fn trigger_for_event(event: String, filter: Option<TriggerFilterYaml>) -> Vec<WorkflowTrigger> {
        match event.as_str() {
            "push" => vec![WorkflowTrigger::Push(
                filter.map(TriggerFilterYaml::into_domain),
            )],
            "pull_request" => vec![WorkflowTrigger::PullRequest(
                filter.map(TriggerFilterYaml::into_domain),
            )],
            "workflow_dispatch" => {
                let inputs = filter
                    .map(TriggerFilterYaml::into_inputs)
                    .unwrap_or_default();
                vec![WorkflowTrigger::Manual { inputs }]
            }
            "schedule" => {
                let expressions = filter
                    .map(TriggerFilterYaml::into_cron_expressions)
                    .unwrap_or_default();
                vec![WorkflowTrigger::Schedule { expressions }]
            }
            _ => Vec::new(),
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
