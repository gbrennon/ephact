use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    domain::value_objects::{ContextValue, JobMatrix},
    workflows::actions::WorkflowYaml,
};

/// The `strategy.matrix:` entry of a job as authored in YAML.
///
/// Matrix variables are written as arbitrary keys alongside the reserved
/// `include` and `exclude` entries, so the variables are flattened.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct JobMatrixYaml {
    #[serde(flatten)]
    variables: HashMap<String, Vec<serde_yaml::Value>>,

    #[serde(default)]
    include: Vec<HashMap<String, serde_yaml::Value>>,

    #[serde(default)]
    exclude: Vec<HashMap<String, serde_yaml::Value>>,
}

impl JobMatrixYaml {
    /// Builds the domain job matrix this YAML describes.
    #[must_use]
    pub fn into_domain(self) -> JobMatrix {
        let variables = self
            .variables
            .into_iter()
            .map(|(name, values)| {
                (
                    name,
                    values
                        .into_iter()
                        .map(WorkflowYaml::context_value_from_yaml)
                        .collect(),
                )
            })
            .collect();
        JobMatrix::new(
            variables,
            Self::map_combinations(self.include),
            Self::map_combinations(self.exclude),
        )
    }

    fn map_combinations(
        combinations: Vec<HashMap<String, serde_yaml::Value>>,
    ) -> Vec<HashMap<String, ContextValue>> {
        combinations
            .into_iter()
            .map(|combination| {
                combination
                    .into_iter()
                    .map(|(name, value)| (name, WorkflowYaml::context_value_from_yaml(value)))
                    .collect()
            })
            .collect()
    }
}
