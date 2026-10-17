use crate::{
    application::{
        dtos::responses::{ContainerConfigResponse, HostInfoResponse},
        ports::outbound::{ContainerPort, ContainerRuntimePort},
    },
    domain::errors::ContainerError,
};

/// Represents an unavailable container runtime for commands that only inspect
/// workflow or action files.
pub struct UnavailableContainerRuntime;

impl ContainerRuntimePort for UnavailableContainerRuntime {
    fn pull_image(&self, _image: &str, _platform: Option<&str>) -> Result<(), ContainerError> {
        Err(ContainerError::NotAvailable)
    }

    fn create_container(
        &self,
        _config: &ContainerConfigResponse,
    ) -> Result<Box<dyn ContainerPort>, ContainerError> {
        Err(ContainerError::NotAvailable)
    }

    fn remove_container(&self, _name: &str) -> Result<(), ContainerError> {
        Err(ContainerError::NotAvailable)
    }

    fn stop_container(&self, _name: &str) -> Result<(), ContainerError> {
        Err(ContainerError::NotAvailable)
    }

    fn kill_container(&self, _name: &str) -> Result<(), ContainerError> {
        Err(ContainerError::NotAvailable)
    }

    fn get_host_info(&self) -> Result<HostInfoResponse, ContainerError> {
        Err(ContainerError::NotAvailable)
    }
}
