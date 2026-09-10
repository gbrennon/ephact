use ephact::{
    application::{
        dtos::{ContainerConfig, HostInfo},
        ports::outbound::{ContainerRuntimePort, container_port::ContainerPort},
    },
    domain::errors::ContainerError,
};

use super::succeeding_container::SucceedingContainer;
use crate::support::container_activity::ContainerActivity;

/// Container runtime of the scenario where every image is available and every
/// command succeeds.
#[derive(Clone)]
pub struct SucceedingRuntime {
    activity: ContainerActivity,
}

impl SucceedingRuntime {
    pub fn recording(activity: ContainerActivity) -> Self {
        Self { activity }
    }
}

impl ContainerRuntimePort for SucceedingRuntime {
    fn pull_image(&self, image: &str, _platform: Option<&str>) -> Result<(), ContainerError> {
        self.activity.record_pulled_image(image);
        Ok(())
    }

    fn create_container(
        &self,
        _config: &ContainerConfig,
    ) -> Result<Box<dyn ContainerPort>, ContainerError> {
        Ok(Box::new(SucceedingContainer::recording(
            self.activity.clone(),
        )))
    }

    fn remove_container(&self, _name: &str) -> Result<(), ContainerError> {
        Ok(())
    }

    fn stop_container(&self, name: &str) -> Result<(), ContainerError> {
        self.activity.record_stopped_container(name);
        Ok(())
    }

    fn kill_container(&self, name: &str) -> Result<(), ContainerError> {
        self.activity.record_killed_container(name);
        Ok(())
    }

    fn get_host_info(&self) -> Result<HostInfo, ContainerError> {
        Ok(HostInfo::new("linux", "x86_64", "e2e"))
    }
}
