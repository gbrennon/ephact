#![allow(dead_code)]
use ephact::application::dtos::ContainerConfig;
use ephact::application::dtos::HostInfo;
use ephact::application::ports::outbound::ContainerRuntimePort;
use ephact::application::ports::outbound::container_port::ContainerPort;
use ephact::domain::errors::ContainerError;

/// Runtime whose every fallible operation fails with a "Docker"-worded error.
///
/// Mirrors bollard, which always names "Docker" in its errors even when the
/// backend is Podman. Lets the adapter's error-rewriting be exercised without a
/// real container runtime.
#[derive(Clone, Default)]
pub struct StubDockerErroringRuntime;

impl StubDockerErroringRuntime {
    fn docker_error() -> ContainerError {
        ContainerError::Internal("Docker daemon refused the request".to_string())
    }
}

impl ContainerRuntimePort for StubDockerErroringRuntime {
    fn pull_image(&self, _image: &str, _platform: Option<&str>) -> Result<(), ContainerError> {
        Err(Self::docker_error())
    }

    fn create_container(
        &self,
        _config: &ContainerConfig,
    ) -> Result<Box<dyn ContainerPort>, ContainerError> {
        Err(Self::docker_error())
    }

    fn remove_container(&self, _name: &str) -> Result<(), ContainerError> {
        Err(Self::docker_error())
    }

    fn stop_container(&self, _name: &str) -> Result<(), ContainerError> {
        Err(Self::docker_error())
    }

    fn kill_container(&self, _name: &str) -> Result<(), ContainerError> {
        Err(Self::docker_error())
    }

    fn get_host_info(&self) -> Result<HostInfo, ContainerError> {
        Err(Self::docker_error())
    }
}
