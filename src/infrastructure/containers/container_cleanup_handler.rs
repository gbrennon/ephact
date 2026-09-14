use std::sync::Arc;

use crate::{
    application::ports::outbound::ContainerRuntimePort, domain::messages::events::DomainEvent,
    infrastructure::messaging::domain_event_handler::DomainEventHandler,
};

/// Infrastructure handler that cleans up containers when execution finishes.
pub struct ContainerCleanupHandler {
    runtime: Arc<dyn ContainerRuntimePort>,
}

impl ContainerCleanupHandler {
    pub fn new(runtime: Arc<dyn ContainerRuntimePort>) -> Self {
        Self { runtime }
    }
}

impl DomainEventHandler for ContainerCleanupHandler {
    /// Brings every container a completed run left behind down with a graceful
    /// stop, force-kills any that did not exit, then removes it. The kill is a
    /// safety net: after a successful stop it is a no-op. Cached images are
    /// never deleted.
    fn handle(&self, event: &DomainEvent) {
        let DomainEvent::ActRunCompleted(payload) = event else {
            return;
        };
        for name in payload.container_names() {
            // Stop container with timeout
            if let Err(e) = self.runtime.stop_container(name) {
                eprintln!("Warning: failed to stop container {}: {:?}", name, e);
            }
            
            // Force kill if stop failed
            if let Err(e) = self.runtime.kill_container(name) {
                eprintln!("Warning: failed to kill container {}: {:?}", name, e);
            }
            
            // Remove container
            if let Err(e) = self.runtime.remove_container(name) {
                eprintln!("Warning: failed to remove container {}: {:?}", name, e);
            }
        }
    }
}
