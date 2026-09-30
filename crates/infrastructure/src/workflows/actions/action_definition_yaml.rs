use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    domain::value_objects::ActionDefinition,
    workflows::actions::{ActionInputYaml, ActionRuntimeYaml},
};

/// An action's `action.yml` as authored in YAML.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ActionDefinitionYaml {
    name: Option<String>,

    #[serde(default)]
    description: Option<String>,

    #[serde(default)]
    inputs: HashMap<String, ActionInputYaml>,

    runs: ActionRuntimeYaml,
}

impl ActionDefinitionYaml {
    /// Returns the optional name authored in the action definition.
    #[must_use]
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Builds the domain action definition this YAML describes.
    #[must_use]
    pub fn into_domain(self) -> ActionDefinition {
        let name = self.name.clone().unwrap_or_default();
        self.into_domain_with_name(name)
    }

    /// Builds the domain definition using a resolved name.
    #[must_use]
    pub fn into_domain_with_name(self, name: impl Into<String>) -> ActionDefinition {
        let inputs = self
            .inputs
            .into_iter()
            .map(|(name, input)| (name, input.into_domain()))
            .collect();
        ActionDefinition::new(name, self.description, inputs, self.runs.into_domain())
    }
}
