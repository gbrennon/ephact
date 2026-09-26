use super::super::{docker::DockerRuntime, podman::PodmanRuntime};
use crate::{
    application::{
        dtos::responses::{ContainerConfigResponse, HostInfoResponse},
        ports::outbound::{ContainerRuntimePort, container_port::ContainerPort},
    },
    domain::errors::ContainerError,
};

/// Strategy-pattern context over a container runtime.
///
/// Holds an injected runtime strategy (the contract is
/// [`ContainerRuntimePort`]) and delegates every operation to it, rewriting the
/// literal "Docker" in error messages with the active runtime's name. Bollard
/// always reports "Docker" even when talking to Podman, so the name is supplied
/// at construction rather than inferred.
pub struct ContainerRuntimeAdapter {
    runtime: Box<dyn ContainerRuntimePort>,
    runtime_name: String,
}

impl ContainerRuntimeAdapter {
    /// Wraps `runtime`, labelling it with `runtime_name` (e.g. "Docker",
    /// "Podman") for error rewriting.
    pub fn new(runtime: Box<dyn ContainerRuntimePort>, runtime_name: String) -> Self {
        Self {
            runtime,
            runtime_name,
        }
    }

    /// Human-readable name of the active runtime (e.g. "Docker", "Podman").
    pub fn runtime_name(&self) -> &str {
        &self.runtime_name
    }

    /// Auto-detect the available container runtime.
    ///
    /// Tries Docker first, then falls back to Podman. Returns
    /// `ContainerError::NotAvailable` (or the Podman connection error) if
    /// neither is reachable.
    pub fn detect() -> Result<Self, ContainerError> {
        if let Ok(runtime) = DockerRuntime::new() {
            return Ok(Self::new(Box::new(runtime), "Docker".to_string()));
        }
        let podman = PodmanRuntime::new()?;
        Ok(Self::new(Box::new(podman), "Podman".to_string()))
    }

    /// Replace "Docker" with the actual runtime name in error messages.
    /// Bollard always reports "Docker" even when talking to Podman.
    fn map_error(&self, err: ContainerError) -> ContainerError {
        if self.runtime_name == "Docker" {
            return err;
        }
        let text = format!("{:?}", err).replace("Docker", &self.runtime_name);
        ContainerError::Internal(text)
    }
}

impl ContainerRuntimePort for ContainerRuntimeAdapter {
    fn pull_image(&self, image: &str, platform: Option<&str>) -> Result<(), ContainerError> {
        self.runtime
            .pull_image(image, platform)
            .map_err(|e| self.map_error(e))
    }

    fn create_container(
        &self,
        config: &ContainerConfigResponse,
    ) -> Result<Box<dyn ContainerPort>, ContainerError> {
        self.runtime
            .create_container(config)
            .map_err(|e| self.map_error(e))
    }

    fn remove_container(&self, name: &str) -> Result<(), ContainerError> {
        self.runtime
            .remove_container(name)
            .map_err(|e| self.map_error(e))
    }

    fn stop_container(&self, name: &str) -> Result<(), ContainerError> {
        self.runtime
            .stop_container(name)
            .map_err(|e| self.map_error(e))
    }

    fn kill_container(&self, name: &str) -> Result<(), ContainerError> {
        self.runtime
            .kill_container(name)
            .map_err(|e| self.map_error(e))
    }

    fn get_host_info(&self) -> Result<HostInfoResponse, ContainerError> {
        self.runtime.get_host_info()
    }
}
