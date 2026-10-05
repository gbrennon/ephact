use std::sync::Arc;

use crate::ports::outbound::container_port::ContainerPort;

/// Request data for resolving a node binary through a container.
pub struct ResolveNodeBinaryRequest {
    container: Arc<dyn ContainerPort>,
}

impl ResolveNodeBinaryRequest {
    /// Creates a request from a container.
    pub fn new(container: Arc<dyn ContainerPort>) -> Self {
        Self { container }
    }

    /// Returns the container.
    pub fn container(&self) -> &dyn ContainerPort {
        self.container.as_ref()
    }
}
