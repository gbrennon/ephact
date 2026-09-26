use serde::Deserialize;

use crate::domain::value_objects::ActionInput;

/// An action input declaration as authored in YAML.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct ActionInputYaml {
    #[serde(default)]
    description: Option<String>,

    #[serde(default)]
    required: bool,

    #[serde(default)]
    default: Option<String>,
}

impl ActionInputYaml {
    /// Builds the domain action input this YAML describes.
    #[must_use]
    pub fn into_domain(self) -> ActionInput {
        ActionInput::new(self.description, self.required, self.default)
    }
}
