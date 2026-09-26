use std::sync::Arc;

use crate::ports::outbound::container_port::ContainerPort;

/// Container a job's steps run in, with the name it was created under.
pub struct PreparedJobContainerResponse {
    container: Arc<dyn ContainerPort>,
    container_name: String,
}

impl PreparedJobContainerResponse {
    /// Creates a new prepared job container.
    pub fn new(container: Box<dyn ContainerPort>, container_name: String) -> Self {
        Self {
            container: Arc::from(container),
            container_name,
        }
    }

    /// Handle to the running container.
    pub fn container(&self) -> &dyn ContainerPort {
        self.container.as_ref()
    }

    pub fn container_handle(&self) -> Arc<dyn ContainerPort> {
        self.container.clone()
    }

    pub fn into_container(self) -> Arc<dyn ContainerPort> {
        self.container
    }

    /// Name the container was created with.
    pub fn container_name(&self) -> &str {
        &self.container_name
    }

    /// Consumes the container and returns the container name.
    pub fn into_container_name(self) -> String {
        self.container_name
    }
}
