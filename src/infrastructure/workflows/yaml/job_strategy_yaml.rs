use serde::Deserialize;

use crate::{domain::value_objects::JobStrategy, infrastructure::workflows::yaml::JobMatrixYaml};

/// The `strategy:` entry of a job as authored in YAML.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct JobStrategyYaml {
    #[serde(default)]
    matrix: Option<JobMatrixYaml>,

    #[serde(rename = "fail-fast")]
    #[serde(default = "default_fail_fast")]
    fail_fast: bool,

    #[serde(rename = "max-parallel")]
    #[serde(default)]
    max_parallel: Option<usize>,
}

impl JobStrategyYaml {
    /// Builds the domain job strategy this YAML describes.
    #[must_use]
    pub fn into_domain(self) -> JobStrategy {
        JobStrategy::new(
            self.matrix.map(JobMatrixYaml::into_domain),
            self.fail_fast,
            self.max_parallel,
        )
    }
}

fn default_fail_fast() -> bool {
    true
}
