/// Request DTO for the
/// [`ContainerCleanupPort`](crate::application::ports::inbound::container_cleanup_port::ContainerCleanupPort)
/// inbound port.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ContainerCleanupRequest {
    /// Containers created during the run that should be cleaned up.
    pub container_names: Vec<String>,
}

impl ContainerCleanupRequest {
    /// Creates a new cleanup request.
    pub fn new(container_names: Vec<String>) -> Self {
        Self { container_names }
    }

    /// Containers created during the run that should be cleaned up.
    pub fn container_names(&self) -> &[String] {
        &self.container_names
    }

    /// Consumes the request and returns the container names.
    pub fn into_container_names(self) -> Vec<String> {
        self.container_names
    }
}
