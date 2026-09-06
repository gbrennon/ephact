#![allow(dead_code)]
use parking_lot::Mutex;
use std::sync::Arc;

use ephact::application::ports::outbound::container_port::ContainerPort;
use ephact::domain::errors::ContainerError;
use ephact::infrastructure::containers::ContainerConfig;
use ephact::infrastructure::containers::ContainerRuntimePort;
use ephact::infrastructure::containers::HostInfo;

use super::stub_container::StubContainer;

#[derive(Clone, Default)]
pub struct SpyContainerRuntime {
    stopped_containers: Arc<Mutex<Vec<String>>>,
    killed_containers: Arc<Mutex<Vec<String>>>,
    removed_containers: Arc<Mutex<Vec<String>>>,
    operations: Arc<Mutex<Vec<String>>>,
}

impl SpyContainerRuntime {
    pub fn new() -> Self {
        Self::default()
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
    fn pull_image(&self, _image: &str, _platform: Option<&str>) -> Result<(), ContainerError> {
        Ok(())
    }

    fn create_container(
        &self,
        _config: &ContainerConfig,
    ) -> Result<Box<dyn ContainerPort>, ContainerError> {
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
        Ok(HostInfo {
            os: "linux".into(),
            arch: "amd64".into(),
            engine_version: "1.0".into(),
        })
    }
}
