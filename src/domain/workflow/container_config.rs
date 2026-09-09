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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_exposes_fields() {
        let config = ContainerConfig::new(
            "ubuntu",
            Some(ContainerCredentials::new("user", "password")),
            HashMap::from([("KEY".into(), "value".into())]),
            vec!["80:80".into()],
            vec!["/tmp:/tmp".into()],
            Some("--privileged".into()),
        );

        assert_eq!(config.image(), "ubuntu");
        assert_eq!(
            config.credentials().map(ContainerCredentials::username),
            Some("user")
        );
        assert_eq!(config.env()["KEY"], "value");
        assert_eq!(config.ports(), &["80:80".to_string()]);
        assert_eq!(config.volumes(), &["/tmp:/tmp".to_string()]);
        assert_eq!(config.options(), Some("--privileged"));
    }
}
