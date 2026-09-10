use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    domain::value_objects::TriggerFilter,
    infrastructure::workflows::yaml::WorkflowDispatchInputYaml,
};

/// Filters attached to one event of a workflow's `on:` mapping.
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
    /// Builds the domain trigger filter this YAML describes.
    #[must_use]
    pub fn into_domain(self) -> TriggerFilter {
        let inputs = self
            .inputs
            .into_iter()
            .map(|(name, input)| (name, input.into_domain()))
            .collect();
        TriggerFilter::new(
            self.branches,
            self.branches_ignore,
            self.tags,
            self.tags_ignore,
            self.paths,
            self.paths_ignore,
            self.types,
            inputs,
            self.cron,
        )
    }
}
