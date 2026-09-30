use serde::Deserialize;

use crate::domain::value_objects::ConcurrencyGroup;

/// The `concurrency:` entry of a workflow or job as authored in YAML.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ConcurrencyGroupYaml {
    group: String,

    #[serde(rename = "cancel-in-progress")]
    cancel_in_progress: Option<bool>,
}

impl ConcurrencyGroupYaml {
    /// Builds the domain concurrency group this YAML describes.
    #[must_use]
    pub fn into_domain(self) -> ConcurrencyGroup {
        ConcurrencyGroup::new(self.group, self.cancel_in_progress)
    }
}
