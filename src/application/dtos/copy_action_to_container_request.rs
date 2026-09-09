use std::path::Path;

use crate::application::ports::outbound::container_port::ContainerPort;

/// Request DTO for the
/// [`CopyActionToContainerPort`](crate::application::ports::inbound::copy_action_to_container_port::CopyActionToContainerPort)
/// inbound port.
pub struct CopyActionToContainerRequest<'a> {
    /// Directory holding the action on the host.
    pub action_dir: &'a Path,
    /// Container the action is copied into.
    pub container: &'a dyn ContainerPort,
}

impl<'a> CopyActionToContainerRequest<'a> {
    /// Creates a new request.
    pub fn new(action_dir: &'a Path, container: &'a dyn ContainerPort) -> Self {
        Self {
            action_dir,
            container,
        }
    }

    /// Directory holding the action on the host.
    pub fn action_dir(&self) -> &'a Path {
        self.action_dir
    }

    /// Container the action is copied into.
    pub fn container(&self) -> &'a dyn ContainerPort {
        self.container
    }
}
