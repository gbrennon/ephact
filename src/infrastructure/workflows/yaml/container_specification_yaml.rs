use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    domain::value_objects::ContainerSpecification,
    infrastructure::workflows::yaml::ContainerCredentialsYaml,
};

/// A job `container:` or `services:` entry as authored in YAML.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ContainerSpecificationYaml {
    image: String,

    #[serde(default)]
    credentials: Option<ContainerCredentialsYaml>,

    #[serde(default)]
    env: HashMap<String, String>,

    #[serde(default)]
    ports: Vec<String>,

    #[serde(default)]
    volumes: Vec<String>,

    #[serde(default)]
    options: Option<String>,
}

impl ContainerSpecificationYaml {
    /// Builds the domain container specification this YAML describes.
    #[must_use]
    pub fn into_domain(self) -> ContainerSpecification {
        ContainerSpecification::new(
            self.image,
            self.credentials.map(ContainerCredentialsYaml::into_domain),
            self.env,
            self.ports,
            self.volumes,
            self.options,
        )
    }
}
