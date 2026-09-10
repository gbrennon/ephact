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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_exposes_all_filters() {
        let input = WorkflowDispatchInput::new(None, false, None, None, Vec::new());
        let config = TriggerFilter::new(
            vec!["main".into()],
            vec!["dev".into()],
            vec!["v1".into()],
            vec!["v2".into()],
            vec!["src/**".into()],
            vec!["docs/**".into()],
            vec!["opened".into()],
            HashMap::from([("name".into(), input)]),
            vec!["0 0 * * *".into()],
        );

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
