use std::collections::HashMap;

use serde::Deserialize;

use crate::domain::value_objects::{RefPattern, TriggerFilter, WorkflowTrigger};

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

#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
struct RefIncludeExcludeYaml {
    #[serde(default)]
    include: Vec<String>,
    #[serde(default)]
    exclude: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, PartialEq)]
#[serde(untagged)]
enum WoodpeckerRefYaml {
    Pattern(StringOrVec),
    IncludeExclude(RefIncludeExcludeYaml),
}

impl WoodpeckerRefYaml {
    fn into_tag_filter(self) -> TriggerFilter {
        match self {
            Self::Pattern(patterns) => patterns
                .into_vec()
                .into_iter()
                .fold(TriggerFilter::new(), |filter, pattern| {
                    filter.with_included_ref(RefPattern::tag(pattern))
                }),
            Self::IncludeExclude(RefIncludeExcludeYaml { include, exclude }) => {
                let filter = include
                    .into_iter()
                    .fold(TriggerFilter::new(), |filter, pattern| {
                        filter.with_included_ref(RefPattern::tag(pattern))
                    });
                exclude.into_iter().fold(filter, |filter, pattern| {
                    filter.with_excluded_ref(RefPattern::tag(pattern))
                })
            }
        }
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct WoodpeckerWhenYaml {
    #[serde(default)]
    event: Option<StringOrVec>,

    #[serde(default, rename = "ref")]
    r#ref: Option<WoodpeckerRefYaml>,
}

impl WoodpeckerWhenYaml {
    #[must_use]
    pub fn into_triggers(self) -> Vec<WorkflowTrigger> {
        let tag_filter = self.r#ref.map(WoodpeckerRefYaml::into_tag_filter);
        self.event
            .map(StringOrVec::into_vec)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|event| Self::trigger_for_event(&event, tag_filter.clone()))
            .collect()
    }

    fn trigger_for_event(
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
