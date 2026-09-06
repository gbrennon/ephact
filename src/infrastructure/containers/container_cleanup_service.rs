use crate::infrastructure::containers::container_cleanup_port::ContainerCleanupPort;
use std::sync::Arc;

use crate::{
    application::dtos::ContainerCleanupRequest,
    infrastructure::containers::container_runtime::ContainerRuntimePort,
};

/// Application service that reacts to workflow completion by cleaning up
/// containers created during the run.
///
/// Implements [`ContainerCleanupPort`] - gracefully stops each container,
/// force-kills any that did not exit (a no-op after a successful stop), and
/// removes it. Does NOT delete cached images.
pub struct ContainerCleanupService {
    runtime: Arc<dyn ContainerRuntimePort>,
}

impl ContainerCleanupService {
    pub fn new(runtime: Arc<dyn ContainerRuntimePort>) -> Self {
        Self { runtime }
    }
}

impl ContainerCleanupPort for ContainerCleanupService {
    fn execute(&self, request: ContainerCleanupRequest) {
        for name in &request.container_names {
            let _ = self.runtime.stop_container(name);
            let _ = self.runtime.kill_container(name);
            let _ = self.runtime.remove_container(name);
        }
    }
}
