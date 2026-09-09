use ephact::application::dtos::ContainerConfig;
use ephact::application::dtos::HostInfo;
use ephact::application::ports::outbound::ContainerRuntimePort;
use ephact::application::ports::outbound::container_port::ContainerPort;
use ephact::domain::errors::ContainerError;

use super::failing_container::FailingContainer;
use crate::support::container_activity::ContainerActivity;

/// Container runtime of the scenario where containers start normally but every
/// command inside them exits with a failure status.
#[derive(Clone)]
pub struct FailingRuntime {
    activity: ContainerActivity,
}

impl FailingRuntime {
    pub fn recording(activity: ContainerActivity) -> Self {
        Self { activity }
    }
}

impl ContainerRuntimePort for FailingRuntime {
    fn pull_image(&self, image: &str, _platform: Option<&str>) -> Result<(), ContainerError> {
        self.activity.record_pulled_image(image);
        Ok(())
    }

    fn create_container(
        &self,
        _config: &ContainerConfig,
    ) -> Result<Box<dyn ContainerPort>, ContainerError> {
        Ok(Box::new(FailingContainer::recording(self.activity.clone())))
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
        Ok(HostInfo::new("linux".into(), "x86_64".into(), "e2e".into()))
    }
}
