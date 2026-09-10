#![allow(dead_code)]

use ephact::application::dtos::responses::ContainerConfigResponse;
use ephact::application::dtos::responses::HostInfoResponse;
use ephact::application::ports::outbound::ContainerRuntimePort;
use ephact::application::ports::outbound::container_port::ContainerPort;
use ephact::domain::errors::ContainerError;
use parking_lot::Mutex;

use super::stub_container::StubContainer;

/// Runtime whose `pull_image` fails for the images it was told to reject,
/// recording every image it was asked to pull.
pub struct StubPullFailingRuntime {
    rejected_images: Vec<String>,
    pub pulled_images: Mutex<Vec<String>>,
}

impl StubPullFailingRuntime {
    pub fn rejecting(rejected_images: Vec<String>) -> Self {
        Self {
            rejected_images,
            pulled_images: Mutex::new(Vec::new()),
        }
    }
}

impl ContainerRuntimePort for StubPullFailingRuntime {
    fn pull_image(&self, image: &str, _platform: Option<&str>) -> Result<(), ContainerError> {
        self.pulled_images.lock().push(image.to_string());
        if self
            .rejected_images
            .iter()
            .any(|rejected| rejected == image)
        {
            return Err(ContainerError::NotAvailable);
        }
        Ok(())
    }

    fn create_container(
        &self,
        _config: &ContainerConfigResponse,
    ) -> Result<Box<dyn ContainerPort>, ContainerError> {
        Ok(Box::new(StubContainer))
    }

    fn remove_container(&self, _name: &str) -> Result<(), ContainerError> {
        Ok(())
    }

    fn stop_container(&self, _name: &str) -> Result<(), ContainerError> {
        Ok(())
    }

    fn kill_container(&self, _name: &str) -> Result<(), ContainerError> {
        Ok(())
    }

    fn get_host_info(&self) -> Result<HostInfoResponse, ContainerError> {
        Ok(HostInfoResponse::new("linux", "amd64", "1.0"))
    }
}
