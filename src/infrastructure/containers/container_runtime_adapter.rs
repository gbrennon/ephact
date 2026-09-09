use crate::application::dtos::ContainerConfig;
use crate::application::dtos::HostInfo;
use crate::application::ports::outbound::ContainerRuntimePort;
use crate::application::ports::outbound::container_port::ContainerPort;
use crate::domain::errors::ContainerError;
use super::docker_runtime::DockerRuntime;
use super::podman_runtime::PodmanRuntime;

/// Strategy-pattern context over a container runtime.
///
/// Holds an injected runtime strategy `R` (the contract is
/// [`ContainerRuntimePort`]) and delegates every operation to it, rewriting the
/// literal "Docker" in error messages with the active runtime's name. Bollard
/// always reports "Docker" even when talking to Podman, so the name is supplied
/// at construction rather than inferred.
///
/// Generic over `R` for static dispatch: production wiring injects a concrete
/// runtime (or a boxed one via [`detect`](Self::detect)), while tests inject a
/// fake implementing [`ContainerRuntimePort`].
pub struct ContainerRuntimeAdapter<R: ContainerRuntimePort> {
    runtime: R,
    runtime_name: String,
}

impl<R: ContainerRuntimePort> ContainerRuntimeAdapter<R> {
    /// Wraps `runtime`, labelling it with `runtime_name` (e.g. "Docker",
    /// "Podman") for error rewriting.
    pub fn new(runtime: R, runtime_name: impl Into<String>) -> Self {
        Self {
            runtime,
            runtime_name: runtime_name.into(),
        }
    }

    /// Human-readable name of the active runtime (e.g. "Docker", "Podman").
    pub fn runtime_name(&self) -> &str {
        &self.runtime_name
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

impl ContainerRuntimeAdapter<Box<dyn ContainerRuntimePort>> {
    /// Auto-detect the available container runtime.
    ///
    /// Tries Docker first, then falls back to Podman. Returns
    /// `ContainerError::NotAvailable` (or the Podman connection error) if
    /// neither is reachable. The chosen backend is boxed so both branches share
    /// a single return type.
    pub fn detect() -> Result<Self, ContainerError> {
        if let Ok(runtime) = DockerRuntime::new() {
            return Ok(Self::new(Box::new(runtime), "Docker"));
        }
        let podman = PodmanRuntime::new()?;
        Ok(Self::new(Box::new(podman), "Podman"))
    }
}

impl<R: ContainerRuntimePort> ContainerRuntimePort for ContainerRuntimeAdapter<R> {
    fn pull_image(&self, image: &str, platform: Option<&str>) -> Result<(), ContainerError> {
        self.runtime
            .pull_image(image, platform)
            .map_err(|e| self.map_error(e))
    }

    fn create_container(
        &self,
        config: &ContainerConfig,
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

    fn get_host_info(&self) -> Result<HostInfo, ContainerError> {
        self.runtime.get_host_info()
    }
}
