use std::collections::HashMap;

use crate::domain::value_objects::WorkflowDispatchInput;

/// Configuration for a specific event type.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct TriggerFilter {
    branches: Vec<String>,
    branches_ignore: Vec<String>,
    tags: Vec<String>,
    tags_ignore: Vec<String>,
    paths: Vec<String>,
    paths_ignore: Vec<String>,
    types: Vec<String>,
    inputs: HashMap<String, WorkflowDispatchInput>,
    cron: Vec<String>,
}

impl TriggerFilter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_branches(mut self, branches: Vec<String>) -> Self {
        self.branches = branches;
        self
    }

    pub fn with_branches_ignore(mut self, branches_ignore: Vec<String>) -> Self {
        self.branches_ignore = branches_ignore;
        self
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn with_tags_ignore(mut self, tags_ignore: Vec<String>) -> Self {
        self.tags_ignore = tags_ignore;
        self
    }

    pub fn with_paths(mut self, paths: Vec<String>) -> Self {
        self.paths = paths;
        self
    }

    pub fn with_paths_ignore(mut self, paths_ignore: Vec<String>) -> Self {
        self.paths_ignore = paths_ignore;
        self
    }

    pub fn with_types(mut self, types: Vec<String>) -> Self {
        self.types = types;
        self
    }

    pub fn with_inputs(mut self, inputs: HashMap<String, WorkflowDispatchInput>) -> Self {
        self.inputs = inputs;
        self
    }

    pub fn with_cron(mut self, cron: Vec<String>) -> Self {
        self.cron = cron;
        self
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_exposes_all_filters() {
        let input = WorkflowDispatchInput::new(None, false, None, None, Vec::new());
        let config = TriggerFilter::new()
            .with_branches(vec!["main".into()])
            .with_branches_ignore(vec!["dev".into()])
            .with_tags(vec!["v1".into()])
            .with_tags_ignore(vec!["v2".into()])
            .with_paths(vec!["src/**".into()])
            .with_paths_ignore(vec!["docs/**".into()])
            .with_types(vec!["opened".into()])
            .with_inputs(HashMap::from([("name".into(), input)]))
            .with_cron(vec!["0 0 * * *".into()]);

        assert_eq!(config.branches(), &["main".to_string()]);
        assert_eq!(config.branches_ignore(), &["dev".to_string()]);
        assert_eq!(config.tags(), &["v1".to_string()]);
        assert_eq!(config.tags_ignore(), &["v2".to_string()]);
        assert_eq!(config.paths(), &["src/**".to_string()]);
        assert_eq!(config.paths_ignore(), &["docs/**".to_string()]);
        assert_eq!(config.types(), &["opened".to_string()]);
        assert!(config.inputs().contains_key("name"));
        assert_eq!(config.cron(), &["0 0 * * *".to_string()]);
    }
}
