use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::ports::outbound::container_port::ContainerPort;

pub struct CopyActionToContainerRequest {
    action_dir: PathBuf,
    container: Arc<dyn ContainerPort>,
}

impl CopyActionToContainerRequest {
    pub fn new(action_dir: PathBuf, container: Arc<dyn ContainerPort>) -> Self {
        Self {
            action_dir,
            container,
        }
    }

    pub fn action_dir(&self) -> &Path {
        &self.action_dir
    }

    pub fn container(&self) -> &dyn ContainerPort {
        self.container.as_ref()
    }
}
