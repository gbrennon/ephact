use super::{container_config::ContainerConfig, host_info::HostInfo};
use crate::{
    application::ports::outbound::container_port::ContainerPort, domain::errors::ContainerError,
};

/// Outbound port for managing a container runtime (Docker, Podman, etc.).
pub trait ContainerRuntimePort: Send + Sync {
    /// Pulls a container image from a registry.
    fn pull_image(&self, image: &str, platform: Option<&str>) -> Result<(), ContainerError>;

    /// Creates a container from the given configuration.
    fn create_container(
        &self,
        config: &ContainerConfig,
    ) -> Result<Box<dyn ContainerPort>, ContainerError>;

    /// Removes a container by name.
    fn remove_container(&self, name: &str) -> Result<(), ContainerError>;

    /// Stops a running container.
    fn stop_container(&self, name: &str) -> Result<(), ContainerError>;

    /// Forcibly kills a running container (sends SIGKILL). Acts as a fallback
    /// after [`stop_container`](Self::stop_container) for containers that did
    /// not shut down gracefully; a missing or already-stopped container is a
    /// no-op. Never removes the container's image.
    fn kill_container(&self, name: &str) -> Result<(), ContainerError>;

    /// Gets information about the host system.
    fn get_host_info(&self) -> Result<HostInfo, ContainerError>;
}
