use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use crate::ports::outbound::container_port::ContainerPort;

/// Inputs for copying an action directory into a container.
pub struct CopyActionToContainerRequest {
    action_dir: PathBuf,
    container: Arc<dyn ContainerPort>,
}

impl CopyActionToContainerRequest {
    /// Creates inputs from an action directory and a container.
    pub fn new(action_dir: PathBuf, container: Arc<dyn ContainerPort>) -> Self {
        Self {
            action_dir,
            container,
        }
    }

    /// Returns the action directory.
    pub fn action_dir(&self) -> &Path {
        &self.action_dir
    }

    /// Returns the container.
    pub fn container(&self) -> &dyn ContainerPort {
        self.container.as_ref()
    }
}
