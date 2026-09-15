use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::{
    application::ports::outbound::ContainerRuntimePort, domain::messages::events::DomainEvent,
    infrastructure::messaging::domain_event_handler::DomainEventHandler,
};

/// Removes a run's containers through the container runtime when the run ends.
///
/// Records each container reported as started for a run and tears the whole set
/// down once that run completes or fails, so no container outlives its run.
pub struct ContainerCleanupHandler {
    runtime: Arc<dyn ContainerRuntimePort>,
    started_containers: Mutex<HashMap<String, Vec<String>>>,
}

impl ContainerCleanupHandler {
    pub fn new(runtime: Arc<dyn ContainerRuntimePort>) -> Self {
        Self {
            runtime,
            started_containers: Mutex::new(HashMap::new()),
        }
    }

    fn record_started_container(&self, run_id: &str, container_name: &str) {
        let mut tracked = self.started_containers.lock().unwrap();
        tracked
            .entry(run_id.to_string())
            .or_default()
            .push(container_name.to_string());
    }

    fn take_started_containers(&self, run_id: &str) -> Vec<String> {
        let mut tracked = self.started_containers.lock().unwrap();
        tracked.remove(run_id).unwrap_or_default()
    }

    fn cleanup_run(&self, run_id: &str, extra_names: &[String]) {
        let mut names = self.take_started_containers(run_id);
        for name in extra_names {
            if !names.contains(name) {
                names.push(name.clone());
            }
        }
        for name in &names {
            self.tear_down_container(name);
        }
    }

    fn tear_down_container(&self, name: &str) {
        let _ = self.runtime.stop_container(name);
        let _ = self.runtime.kill_container(name);
        let _ = self.runtime.remove_container(name);
    }
}

impl DomainEventHandler for ContainerCleanupHandler {
    /// Records a started container, or removes every container of a run once
    /// that run completes or fails.
    fn handle(&self, event: &DomainEvent) {
        match event {
            DomainEvent::ContainerStarted(payload) => {
                self.record_started_container(payload.run_id(), payload.container_name())
            }
            DomainEvent::ActRunCompleted(payload) => {
                self.cleanup_run(payload.run_id(), payload.container_names())
            }
            DomainEvent::RunFailed(payload) => self.cleanup_run(payload.run_id(), &[]),
            _ => {}
        }
    }
}
