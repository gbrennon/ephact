use ephact::application::ports::outbound::ContainerRuntimePort;
use ephact::domain::errors::ContainerError;
use ephact::domain::messages::events::{
    ActRunCompletedPayload, ContainerStartedPayload, DomainEvent, RunFailedPayload,
};
use ephact::infrastructure::containers::container_cleanup_handler::ContainerCleanupHandler;
use ephact::infrastructure::messaging::domain_event_handler::DomainEventHandler;
use std::sync::{Arc, Mutex};

/// Test that verifies container cleanup handler attempts to remove containers
#[test]
fn cleanup_handler_attempts_to_remove_containers_from_completed_event() {
    // Arrange: Create a spy runtime that tracks what methods were called
    let spy_runtime = Arc::new(SpyContainerRuntime::new());
    let handler = ContainerCleanupHandler::new(spy_runtime.clone());

    let container_names = vec!["ephemeral-act-test-12345-1000".to_string()];

    // Act: Publish ActRunCompleted event
    let event = DomainEvent::ActRunCompleted(ActRunCompletedPayload::new(
        "test-run".to_string(),
        "/test/repo".to_string(),
        container_names.clone(),
        true,
    ));

    handler.handle(&event);

    // Assert: Handler should have attempted to clean up
    let calls = spy_runtime.calls.lock().unwrap();
    assert!(
        calls.iter().any(|c| c.starts_with("stop_container")),
        "Handler should call stop_container"
    );
    assert!(
        calls.iter().any(|c| c.starts_with("kill_container")),
        "Handler should call kill_container"
    );
    assert!(
        calls.iter().any(|c| c.starts_with("remove_container")),
        "Handler should call remove_container"
    );

    // Additional assertion: If this test passes but containers still exist
    // on the system, it means the container runtime methods are failing
    // silently (errors are being ignored by `let _ =`)
}

#[test]
fn cleanup_handler_removes_started_containers_when_run_fails() {
    let spy_runtime = Arc::new(SpyContainerRuntime::new());
    let handler = ContainerCleanupHandler::new(spy_runtime.clone());
    let first_container = "ephemeral-act-build-1000-1";
    let second_container = "ephemeral-act-test-1000-2";

    handler.handle(&container_started("run-7", first_container));
    handler.handle(&container_started("run-7", second_container));
    handler.handle(&run_failed("run-7"));

    let calls = spy_runtime.calls.lock().unwrap();
    assert!(calls.contains(&format!("remove_container({})", first_container)));
    assert!(calls.contains(&format!("remove_container({})", second_container)));
}

#[test]
fn cleanup_handler_ignores_started_containers_from_other_runs_when_run_fails() {
    let spy_runtime = Arc::new(SpyContainerRuntime::new());
    let handler = ContainerCleanupHandler::new(spy_runtime.clone());

    handler.handle(&container_started("run-a", "ephemeral-act-a-1000-1"));
    handler.handle(&run_failed("run-b"));

    let calls = spy_runtime.calls.lock().unwrap();
    assert!(!calls.iter().any(|c| c.starts_with("remove_container")));
}

fn container_started(run_id: &str, container_name: &str) -> DomainEvent {
    DomainEvent::ContainerStarted(ContainerStartedPayload::new(
        run_id.to_string(),
        container_name.to_string(),
    ))
}

fn run_failed(run_id: &str) -> DomainEvent {
    DomainEvent::RunFailed(RunFailedPayload::new(
        run_id.to_string(),
        "/test/repo".to_string(),
        None,
        "boom".to_string(),
    ))
}

/// Spy runtime that tracks all method calls
struct SpyContainerRuntime {
    calls: Mutex<Vec<String>>,
}

impl SpyContainerRuntime {
    fn new() -> Self {
        Self {
            calls: Mutex::new(Vec::new()),
        }
    }
}

impl ContainerRuntimePort for SpyContainerRuntime {
    fn create_container(
        &self,
        _config: &ephact::application::dtos::responses::ContainerConfigResponse,
    ) -> Result<
        Box<dyn ephact::application::ports::outbound::container_port::ContainerPort>,
        ContainerError,
    > {
        self.calls
            .lock()
            .unwrap()
            .push("create_container".to_string());
        Err(ContainerError::Internal(
            "not implemented in test".to_string(),
        ))
    }

    fn stop_container(&self, name: &str) -> Result<(), ContainerError> {
        self.calls
            .lock()
            .unwrap()
            .push(format!("stop_container({})", name));
        Ok(())
    }

    fn kill_container(&self, name: &str) -> Result<(), ContainerError> {
        self.calls
            .lock()
            .unwrap()
            .push(format!("kill_container({})", name));
        Ok(())
    }

    fn remove_container(&self, name: &str) -> Result<(), ContainerError> {
        self.calls
            .lock()
            .unwrap()
            .push(format!("remove_container({})", name));
        Ok(())
    }

    fn pull_image(&self, _image: &str, _platform: Option<&str>) -> Result<(), ContainerError> {
        Err(ContainerError::Internal(
            "not implemented in test".to_string(),
        ))
    }

    fn get_host_info(
        &self,
    ) -> Result<ephact::application::dtos::responses::HostInfoResponse, ContainerError> {
        Err(ContainerError::Internal("not implemented".to_string()))
    }
}
