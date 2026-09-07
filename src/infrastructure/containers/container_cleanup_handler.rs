use std::sync::Arc;

use crate::{
    application::ports::outbound::ContainerRuntimePort,
    domain::events::{ActRunCompletedPayload, DomainEvent},
};

/// Infrastructure handler that cleans up containers when execution finishes.
pub struct ContainerCleanupHandler {
    runtime: Arc<dyn ContainerRuntimePort>,
}

impl ContainerCleanupHandler {
    pub fn new(runtime: Arc<dyn ContainerRuntimePort>) -> Self {
        Self { runtime }
    }

    /// Brings every container a completed run left behind down with a graceful
    /// stop, force-kills any that did not exit, then removes it. The kill is a
    /// safety net: after a successful stop it is a no-op. Cached images are
    /// never deleted.
    pub fn handle(&self, event: &DomainEvent) {
        let DomainEvent::ActRunCompleted(ActRunCompletedPayload {
            container_names, ..
        }) = event;
        for name in container_names {
            let _ = self.runtime.stop_container(name);
            let _ = self.runtime.kill_container(name);
            let _ = self.runtime.remove_container(name);
        }
    }
}
