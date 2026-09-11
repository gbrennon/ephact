#![allow(dead_code)]
use ephact::application::dtos::responses::ContainerConfigResponse;
use ephact::application::dtos::responses::HostInfoResponse;
use ephact::application::ports::outbound::ContainerRuntimePort;
use ephact::application::ports::outbound::container_port::ContainerPort;
use ephact::domain::errors::ContainerError;

#[derive(Clone, Default)]
pub struct StubFailingContainerRuntime;

impl ContainerRuntimePort for StubFailingContainerRuntime {
    fn pull_image(&self, _image: &str, _platform: Option<&str>) -> Result<(), ContainerError> {
        Err(ContainerError::NotAvailable)
    }

    fn create_container(
        &self,
        _config: &ContainerConfigResponse,
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

    fn get_host_info(&self) -> Result<HostInfoResponse, ContainerError> {
        Err(ContainerError::NotAvailable)
    }
}
