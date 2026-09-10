use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    domain::value_objects::ActionDefinition,
    infrastructure::workflows::yaml::{ActionInputYaml, ActionRuntimeYaml},
};

/// An action's `action.yml` as authored in YAML.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ActionDefinitionYaml {
    name: String,

    #[serde(default)]
    description: Option<String>,

    #[serde(default)]
    inputs: HashMap<String, ActionInputYaml>,

    runs: ActionRuntimeYaml,
}

impl ActionDefinitionYaml {
    /// Builds the domain action definition this YAML describes.
    #[must_use]
    pub fn into_domain(self) -> ActionDefinition {
        let inputs = self
            .inputs
            .into_iter()
            .map(|(name, input)| (name, input.into_domain()))
            .collect();
        ActionDefinition::new(self.name, self.description, inputs, self.runs.into_domain())
    }
}
