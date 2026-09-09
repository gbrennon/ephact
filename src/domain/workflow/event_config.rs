use std::collections::HashMap;

use serde::Deserialize;

use super::WorkflowDispatchInput;

/// Configuration for a specific event type.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct EventConfig {
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
    inputs: HashMap<String, WorkflowDispatchInput>,

    #[serde(default)]
    cron: Vec<String>,
}

impl EventConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        branches: Vec<String>,
        branches_ignore: Vec<String>,
        tags: Vec<String>,
        tags_ignore: Vec<String>,
        paths: Vec<String>,
        paths_ignore: Vec<String>,
        types: Vec<String>,
        inputs: HashMap<String, WorkflowDispatchInput>,
        cron: Vec<String>,
    ) -> Self {
        Self {
            branches,
            branches_ignore,
            tags,
            tags_ignore,
            paths,
            paths_ignore,
            types,
            inputs,
            cron,
        }
    }

    pub fn branches(&self) -> &[String] {
        &self.branches
    }

    pub fn branches_ignore(&self) -> &[String] {
        &self.branches_ignore
    }

    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    pub fn tags_ignore(&self) -> &[String] {
        &self.tags_ignore
    }

    pub fn paths(&self) -> &[String] {
        &self.paths
    }

    pub fn paths_ignore(&self) -> &[String] {
        &self.paths_ignore
    }

    pub fn types(&self) -> &[String] {
        &self.types
    }

    pub fn inputs(&self) -> &HashMap<String, WorkflowDispatchInput> {
        &self.inputs
    }

    pub fn cron(&self) -> &[String] {
        &self.cron
    }
}
