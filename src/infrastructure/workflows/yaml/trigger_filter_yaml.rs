use std::collections::HashMap;

use serde::Deserialize;

use super::WorkflowDispatchInputYaml;
use crate::domain::value_objects::{RefPattern, TriggerFilter, TriggerInput};

#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct TriggerFilterYaml {
    #[serde(default)]
    branches: Vec<String>,
    #[serde(rename = "branches-ignore")]
    #[serde(default)]
    branches_ignore: Vec<String>,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(rename = "tags-ignore")]
    #[serde(default)]
    tags_ignore: Vec<String>,
    #[serde(default)]
    paths: Vec<String>,
    #[serde(rename = "paths-ignore")]
    #[serde(default)]
    paths_ignore: Vec<String>,
    #[serde(default)]
    types: Vec<String>,
    #[serde(default)]
    inputs: HashMap<String, WorkflowDispatchInputYaml>,
    #[serde(default)]
    cron: Vec<String>,
}

impl TriggerFilterYaml {
    pub fn into_domain(self) -> TriggerFilter {
        let mut filter = TriggerFilter::new().with_event_types(self.types);
        for pattern in self.branches {
            filter = filter.with_included_ref(RefPattern::branch(pattern));
        }
        for pattern in self.tags {
            filter = filter.with_included_ref(RefPattern::tag(pattern));
        }
        for pattern in self.paths {
            filter = filter.with_included_ref(RefPattern::path(pattern));
        }
        for pattern in self.branches_ignore {
            filter = filter.with_excluded_ref(RefPattern::branch(pattern));
        }
        for pattern in self.tags_ignore {
            filter = filter.with_excluded_ref(RefPattern::tag(pattern));
        }
        for pattern in self.paths_ignore {
            filter = filter.with_excluded_ref(RefPattern::path(pattern));
        }
        filter
    }

    pub fn into_inputs(self) -> HashMap<String, TriggerInput> {
        self.inputs
            .into_iter()
            .map(|(name, input)| (name, input.into_domain()))
            .collect()
    }

    pub fn into_cron_expressions(self) -> Vec<String> {
        self.cron
    }
}
