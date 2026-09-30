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
        let included_refs = self
            .branches
            .into_iter()
            .map(RefPattern::branch)
            .chain(self.tags.into_iter().map(RefPattern::tag))
            .chain(self.paths.into_iter().map(RefPattern::path));
        let excluded_refs = self
            .branches_ignore
            .into_iter()
            .map(RefPattern::branch)
            .chain(self.tags_ignore.into_iter().map(RefPattern::tag))
            .chain(self.paths_ignore.into_iter().map(RefPattern::path));
        let filter = TriggerFilter::new().with_event_types(self.types);
        let filter = included_refs.fold(filter, |f, r| f.with_included_ref(r));
        excluded_refs.fold(filter, |f, r| f.with_excluded_ref(r))
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
