use std::sync::Arc;

use crate::application::ports::outbound::container_port::ContainerPort;

/// Container a job's steps run in, with the name it was created under.
pub struct PreparedJobContainer {
    /// Handle to the running container.
    container: Arc<dyn ContainerPort>,
    /// Name the container was created with.
    container_name: String,
}

impl PreparedJobContainer {
    /// Creates a new prepared job container.
    pub fn new(container: Arc<dyn ContainerPort>, container_name: String) -> Self {
        Self {
            container,
            container_name,
        }
    }

    /// Handle to the running container.
    pub fn container(&self) -> &Arc<dyn ContainerPort> {
        &self.container
    }

    /// Cloned handle to the running container.
    pub fn container_arc(&self) -> Arc<dyn ContainerPort> {
        self.container.clone()
    }

    /// Consumes the container and returns the handle.
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
