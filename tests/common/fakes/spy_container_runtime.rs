#![allow(dead_code)]
use parking_lot::Mutex;
use std::sync::Arc;

use ephact::application::dtos::ContainerConfig;
use ephact::application::dtos::HostInfo;
use ephact::application::ports::outbound::ContainerRuntimePort;
use ephact::application::ports::outbound::container_port::ContainerPort;
use ephact::domain::errors::ContainerError;

use super::stub_container::StubContainer;

#[derive(Clone, Default)]
pub struct SpyContainerRuntime {
    pulled_images: Arc<Mutex<Vec<String>>>,
    created_containers: Arc<Mutex<Vec<String>>>,
    stopped_containers: Arc<Mutex<Vec<String>>>,
    killed_containers: Arc<Mutex<Vec<String>>>,
    removed_containers: Arc<Mutex<Vec<String>>>,
    operations: Arc<Mutex<Vec<String>>>,
}

impl SpyContainerRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pulled_images(&self) -> Vec<String> {
        self.pulled_images.lock().clone()
    }

    pub fn created_containers(&self) -> Vec<String> {
        self.created_containers.lock().clone()
    }

    pub fn stopped_containers(&self) -> Vec<String> {
        self.stopped_containers.lock().clone()
    }

    pub fn killed_containers(&self) -> Vec<String> {
        self.killed_containers.lock().clone()
    }

    pub fn removed_containers(&self) -> Vec<String> {
        self.removed_containers.lock().clone()
    }

    /// Every stop/kill/remove call in the order it was issued, as
    /// `"<operation>:<name>"` entries (e.g. `"stop:app1"`).
    pub fn operations(&self) -> Vec<String> {
        self.operations.lock().clone()
    }
}

impl ContainerRuntimePort for SpyContainerRuntime {
    fn pull_image(&self, image: &str, _platform: Option<&str>) -> Result<(), ContainerError> {
        self.pulled_images.lock().push(image.to_string());
        self.operations.lock().push(format!("pull:{image}"));
        Ok(())
    }

    fn create_container(
        &self,
        config: &ContainerConfig,
    ) -> Result<Box<dyn ContainerPort>, ContainerError> {
        let name = config.name().clone().unwrap_or_default();
        self.created_containers.lock().push(name.to_owned());
        self.operations.lock().push(format!("create:{name}"));
        Ok(Box::new(StubContainer))
    }

    fn remove_container(&self, name: &str) -> Result<(), ContainerError> {
        self.removed_containers.lock().push(name.to_string());
        self.operations.lock().push(format!("remove:{name}"));
        Ok(())
    }

    fn stop_container(&self, name: &str) -> Result<(), ContainerError> {
        self.stopped_containers.lock().push(name.to_string());
        self.operations.lock().push(format!("stop:{name}"));
        Ok(())
    }

    fn kill_container(&self, name: &str) -> Result<(), ContainerError> {
        self.killed_containers.lock().push(name.to_string());
        self.operations.lock().push(format!("kill:{name}"));
        Ok(())
    }

    fn get_host_info(&self) -> Result<HostInfo, ContainerError> {
        Ok(HostInfo::new("linux", "amd64", "1.0"))
    }
}
