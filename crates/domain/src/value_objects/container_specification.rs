use std::collections::HashMap;

/// Configuration for a container used by a job or service.
#[derive(Debug, Clone, PartialEq)]
pub struct ContainerSpecification {
    image: String,
    env: HashMap<String, String>,
    ports: Vec<String>,
    volumes: Vec<String>,
    options: Option<String>,
}

impl ContainerSpecification {
    pub fn new(image: impl Into<String>) -> Self {
        Self {
            image: image.into(),
            env: HashMap::new(),
            ports: Vec::new(),
            volumes: Vec::new(),
            options: None,
        }
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
    fn new_sets_image() {
        let config = ContainerSpecification::new("ubuntu");

        assert_eq!(config.image(), "ubuntu");
    }

    #[test]
    fn with_env_sets_environment() {
        let config = ContainerSpecification::new("ubuntu")
            .with_env(HashMap::from([("KEY".into(), "value".into())]));

        assert_eq!(config.env()["KEY"], "value");
    }

    #[test]
    fn with_ports_sets_ports() {
        let config = ContainerSpecification::new("ubuntu").with_ports(vec!["80:80".into()]);

        assert_eq!(config.ports(), &["80:80".to_string()]);
    }

    #[test]
    fn with_volumes_sets_volumes() {
        let config = ContainerSpecification::new("ubuntu").with_volumes(vec!["/tmp:/tmp".into()]);

        assert_eq!(config.volumes(), &["/tmp:/tmp".to_string()]);
    }

    #[test]
    fn with_options_sets_options() {
        let config =
            ContainerSpecification::new("ubuntu").with_options(Some("--privileged".into()));

        assert_eq!(config.options(), Some("--privileged"));
    }
}
