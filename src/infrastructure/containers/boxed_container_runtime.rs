use crate::application::dtos::{ContainerConfig, HostInfo};
use crate::{
    application::ports::outbound::{ContainerRuntimePort, container_port::ContainerPort},
    domain::errors::ContainerError,
};

/// Lets a boxed runtime satisfy [`ContainerRuntimePort`] so a runtime-selected
/// backend (see [`ContainerRuntimeAdapter::detect`]) can be stored behind the
/// generic strategy bound. Delegates every call to the inner value.
///
/// [`ContainerRuntimeAdapter::detect`]: super::container_runtime_adapter::ContainerRuntimeAdapter::detect
impl ContainerRuntimePort for Box<dyn ContainerRuntimePort> {
    fn pull_image(&self, image: &str, platform: Option<&str>) -> Result<(), ContainerError> {
        (**self).pull_image(image, platform)
    }

    fn create_container(
        &self,
        config: &ContainerConfig,
    ) -> Result<Box<dyn ContainerPort>, ContainerError> {
        (**self).create_container(config)
    }

    fn remove_container(&self, name: &str) -> Result<(), ContainerError> {
        (**self).remove_container(name)
    }

    fn stop_container(&self, name: &str) -> Result<(), ContainerError> {
        (**self).stop_container(name)
    }

    fn kill_container(&self, name: &str) -> Result<(), ContainerError> {
        (**self).kill_container(name)
    }

    fn get_host_info(&self) -> Result<HostInfo, ContainerError> {
        (**self).get_host_info()
    }
}
