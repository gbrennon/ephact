use crate::traits::NetworkCommandClassifier;

#[derive(Default)]
pub struct StubClassifier {
    package_registry: bool,
    http_request: bool,
    network_command: bool,
    remote_mutation: bool,
}

impl StubClassifier {
    /// Creates a classifier that reports package and HTTP network access.
    pub fn package_registry_http_request_network_command() -> Self {
        Self {
            package_registry: true,
            http_request: true,
            network_command: true,
            ..Self::default()
        }
    }

    /// Creates a classifier that reports HTTP and command network access.
    pub fn http_request_and_network_command() -> Self {
        Self {
            http_request: true,
            network_command: true,
            ..Self::default()
        }
    }

    /// Creates a classifier that reports command network access.
    pub fn network_command() -> Self {
        Self {
            network_command: true,
            ..Self::default()
        }
    }

    /// Creates a classifier that reports HTTP network access.
    pub fn http_request() -> Self {
        Self {
            http_request: true,
            ..Self::default()
        }
    }

    /// Creates a classifier that reports remote mutation.
    pub fn remote_mutation() -> Self {
        Self {
            remote_mutation: true,
            ..Self::default()
        }
    }
}

impl NetworkCommandClassifier for StubClassifier {
    fn accesses_package_registry(&self, _script: &str) -> bool {
        self.package_registry
    }

    fn issues_http_request(&self, _script: &str) -> bool {
        self.http_request
    }

    fn uses_network_command(&self, _script: &str) -> bool {
        self.network_command
    }

    fn mutates_remote_environment(&self, _script: &str) -> bool {
        self.remote_mutation
    }
}
