use serde::Deserialize;

use crate::domain::value_objects::ContainerCredentials;

/// Registry credentials of a container entry as authored in YAML.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ContainerCredentialsYaml {
    username: String,
    password: String,
}

impl ContainerCredentialsYaml {
    /// Builds the domain container credentials this YAML describes.
    #[must_use]
    pub fn into_domain(self) -> ContainerCredentials {
        ContainerCredentials::new(self.username, self.password)
    }
}
