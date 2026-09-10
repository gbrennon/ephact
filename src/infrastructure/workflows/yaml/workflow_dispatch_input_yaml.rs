use serde::Deserialize;

use crate::domain::value_objects::WorkflowDispatchInput;

/// Input declaration of a `workflow_dispatch` trigger as authored in YAML.
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct WorkflowDispatchInputYaml {
    description: Option<String>,

    #[serde(default)]
    required: bool,

    #[serde(default)]
    default: Option<String>,

    #[serde(rename = "type")]
    #[serde(default)]
    input_type: Option<String>,

    #[serde(default)]
    options: Vec<String>,
}

impl WorkflowDispatchInputYaml {
    /// Builds the domain dispatch input this YAML describes.
    #[must_use]
    pub fn into_domain(self) -> WorkflowDispatchInput {
        WorkflowDispatchInput::new(
            self.description,
            self.required,
            self.default,
            self.input_type,
            self.options,
        )
    }
}
