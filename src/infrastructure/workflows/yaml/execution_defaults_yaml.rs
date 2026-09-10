use serde::Deserialize;

use crate::{
    domain::value_objects::ExecutionDefaults, infrastructure::workflows::yaml::RunStepDefaultsYaml,
};

/// The `defaults:` entry of a workflow or job as authored in YAML.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct ExecutionDefaultsYaml {
    run: Option<RunStepDefaultsYaml>,
}

impl ExecutionDefaultsYaml {
    /// Builds the domain execution defaults this YAML describes.
    #[must_use]
    pub fn into_domain(self) -> ExecutionDefaults {
        ExecutionDefaults::new(self.run.map(RunStepDefaultsYaml::into_domain))
    }
}
