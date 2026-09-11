use crate::application::ports::outbound::container_port::ContainerPort;

/// Container a job's steps run in, with the name it was created under.
pub struct PreparedJobContainerResponse {
    /// Handle to the running container.
    container: Box<dyn ContainerPort>,
    /// Name the container was created with.
    container_name: String,
}

impl PreparedJobContainerResponse {
    /// Creates a new prepared job container.
    pub fn new(container: Box<dyn ContainerPort>, container_name: String) -> Self {
        Self {
            container,
            container_name,
        }
    }

    /// Handle to the running container.
    pub fn container(&self) -> &dyn ContainerPort {
        &*self.container
    }

    /// Consumes the container and returns the handle.
    pub fn into_container(self) -> Box<dyn ContainerPort> {
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
