use std::collections::HashMap;

use serde::Deserialize;

use crate::domain::value_objects::{RefPattern, TriggerFilter, WorkflowTrigger};
/// Woodpecker event and reference filters for a pipeline trigger.
///
/// Event and reference values are retained as YAML values until conversion
/// into the domain trigger model.

#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct WoodpeckerWhenYaml {
    #[serde(default)]
    event: Option<serde_yaml::Value>,

    #[serde(default, rename = "ref")]
    r#ref: Option<serde_yaml::Value>,
}

impl WoodpeckerWhenYaml {
    /// Parses Woodpecker trigger conditions from YAML content.
    pub fn parse(content: &str) -> Result<Self, serde_yaml::Error> {
        serde_yaml::from_str(content)
    }

    /// Converts the trigger conditions into domain workflow triggers.
    pub fn into_triggers(self) -> Vec<WorkflowTrigger> {
        let tag_filter = self.tag_filter();
        let events = self
            .event
            .as_ref()
            .map(|event| self.strings_from_value(event))
            .unwrap_or_default();
        events
            .into_iter()
            .filter_map(|event| self.trigger_for_event(&event, tag_filter.clone()))
            .collect()
    }

    fn tag_filter(&self) -> Option<TriggerFilter> {
        let value = self.r#ref.as_ref()?;
        match value {
            serde_yaml::Value::String(_) | serde_yaml::Value::Sequence(_) => Some(
                self.strings_from_value(value)
                    .into_iter()
                    .fold(TriggerFilter::new(), |filter, pattern| {
                        filter.with_included_ref(RefPattern::tag(pattern))
                    }),
            ),
            serde_yaml::Value::Mapping(values) => {
                let include = values
                    .get("include")
                    .map(|value| self.strings_from_value(value))
                    .unwrap_or_default();
                let exclude = values
                    .get("exclude")
                    .map(|value| self.strings_from_value(value))
                    .unwrap_or_default();
                let filter = include
                    .into_iter()
                    .fold(TriggerFilter::new(), |filter, pattern| {
                        filter.with_included_ref(RefPattern::tag(pattern))
                    });
                Some(exclude.into_iter().fold(filter, |filter, pattern| {
                    filter.with_excluded_ref(RefPattern::tag(pattern))
                }))
            }
            _ => None,
        }
    }

    fn strings_from_value(&self, value: &serde_yaml::Value) -> Vec<String> {
        match value {
            serde_yaml::Value::String(value) => vec![value.clone()],
            serde_yaml::Value::Sequence(values) => values
                .iter()
                .filter_map(serde_yaml::Value::as_str)
                .map(str::to_owned)
                .collect(),
            _ => Vec::new(),
        }
    }

    fn trigger_for_event(
        &self,
        event: &str,
        tag_filter: Option<TriggerFilter>,
    ) -> Option<WorkflowTrigger> {
        match event {
            "push" => Some(WorkflowTrigger::Push(None)),
            "pull_request" => Some(WorkflowTrigger::PullRequest(None)),
            "tag" => Some(WorkflowTrigger::Tag(tag_filter)),
            "manual" => Some(WorkflowTrigger::Manual {
                inputs: HashMap::new(),
            }),
            _ => None,
        }
    }
}
