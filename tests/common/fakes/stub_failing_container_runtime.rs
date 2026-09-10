#![allow(dead_code)]
use ephact::{
    application::{
        dtos::{ContainerConfig, HostInfo},
        ports::outbound::{ContainerRuntimePort, container_port::ContainerPort},
    },
    domain::errors::ContainerError,
};

#[derive(Clone, Default)]
pub struct StubFailingContainerRuntime;

impl ContainerRuntimePort for StubFailingContainerRuntime {
    fn pull_image(&self, _image: &str, _platform: Option<&str>) -> Result<(), ContainerError> {
        Err(ContainerError::NotAvailable)
    }

    fn create_container(
        &self,
        _config: &ContainerConfig,
    ) -> Result<Box<dyn ContainerPort>, ContainerError> {
        Err(ContainerError::NotAvailable)
    }

    fn remove_container(&self, name: &str) -> Result<(), ContainerError> {
        Err(ContainerError::RemovalFailed(
            name.to_string(),
            "removal failure".to_string(),
        ))
    }

    fn stop_container(&self, name: &str) -> Result<(), ContainerError> {
        Err(ContainerError::Internal(format!("failed to stop {name}")))
    }

    fn kill_container(&self, name: &str) -> Result<(), ContainerError> {
        Err(ContainerError::KillFailed(
            name.to_string(),
            "kill failure".to_string(),
        ))
    }

    fn get_host_info(&self) -> Result<HostInfo, ContainerError> {
        Err(ContainerError::NotAvailable)
    }
}
