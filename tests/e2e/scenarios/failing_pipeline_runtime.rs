use ephact::{
    application::{
        dtos::responses::{ContainerConfigResponse, HostInfoResponse},
        ports::outbound::{ContainerRuntimePort, container_port::ContainerPort},
    },
    domain::errors::ContainerError,
};

use super::failing_pipeline_container::FailingPipelineContainer;
use crate::support::container_activity::ContainerActivity;

#[derive(Clone)]
pub struct FailingPipelineRuntime {
    activity: ContainerActivity,
}

impl FailingPipelineRuntime {
    pub fn recording(activity: ContainerActivity) -> Self {
        Self { activity }
    }
}

impl ContainerRuntimePort for FailingPipelineRuntime {
    fn pull_image(&self, image: &str, _platform: Option<&str>) -> Result<(), ContainerError> {
        self.activity.record_pulled_image(image);
        Ok(())
    }

    fn create_container(
        &self,
        _config: &ContainerConfigResponse,
    ) -> Result<Box<dyn ContainerPort>, ContainerError> {
        Ok(Box::new(FailingPipelineContainer::recording(
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

    fn get_host_info(&self) -> Result<HostInfoResponse, ContainerError> {
        Ok(HostInfoResponse::new("linux", "x86_64", "e2e"))
    }
}
