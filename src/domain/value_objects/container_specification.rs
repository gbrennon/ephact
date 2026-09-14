use std::collections::HashMap;

use crate::domain::value_objects::ContainerCredentials;

/// Configuration for a container used by a job or service.
#[derive(Debug, Clone, PartialEq)]
pub struct ContainerSpecification {
    image: String,
    credentials: Option<ContainerCredentials>,
    env: HashMap<String, String>,
    ports: Vec<String>,
    volumes: Vec<String>,
    options: Option<String>,
}

impl ContainerSpecification {
    pub fn new(image: impl Into<String>) -> Self {
        Self {
            image: image.into(),
            credentials: None,
            env: HashMap::new(),
            ports: Vec::new(),
            volumes: Vec::new(),
            options: None,
        }
    }

    pub fn with_credentials(mut self, credentials: Option<ContainerCredentials>) -> Self {
        self.credentials = credentials;
        self
    }
    pub fn with_env(mut self, env: HashMap<String, String>) -> Self {
        self.env = env;
        self
    }

    pub fn with_ports(mut self, ports: Vec<String>) -> Self {
        self.ports = ports;
        self
    }

    pub fn with_volumes(mut self, volumes: Vec<String>) -> Self {
        self.volumes = volumes;
        self
    }

    pub fn with_options(mut self, options: Option<String>) -> Self {
        self.options = options;
        self
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
        let config = ContainerSpecification::new("ubuntu")
            .with_credentials(Some(ContainerCredentials::new("user", "password")))
            .with_env(HashMap::from([("KEY".into(), "value".into())]))
            .with_ports(vec!["80:80".into()])
            .with_volumes(vec!["/tmp:/tmp".into()])
            .with_options(Some("--privileged".into()));

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
