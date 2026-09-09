use std::collections::HashMap;

use serde::Deserialize;

use super::ContainerCredentials;

/// Configuration for a container used by a job or service.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct ContainerConfig {
    image: String,

    #[serde(default)]
    credentials: Option<ContainerCredentials>,

    #[serde(default)]
    env: HashMap<String, String>,

    #[serde(default)]
    ports: Vec<String>,

    #[serde(default)]
    volumes: Vec<String>,

    #[serde(default)]
    options: Option<String>,
}

impl ContainerConfig {
    pub fn new(
        image: impl Into<String>,
        credentials: Option<ContainerCredentials>,
        env: HashMap<String, String>,
        ports: Vec<String>,
        volumes: Vec<String>,
        options: Option<String>,
    ) -> Self {
        Self {
            image: image.into(),
            credentials,
            env,
            ports,
            volumes,
            options,
        }
    }

    pub fn image(&self) -> &str {
        &self.image
    }

    pub fn credentials(&self) -> Option<&ContainerCredentials> {
        self.credentials.as_ref()
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn ports(&self) -> &[String] {
        &self.ports
    }

    pub fn volumes(&self) -> &[String] {
        &self.volumes
    }

    pub fn options(&self) -> Option<&str> {
        self.options.as_deref()
    }
}
