use std::sync::Arc;

use ephact::{
    application::ports::outbound::DomainEventPublisherPort,
    domain::messages::{commands::Event, events::WorkflowRunCompletedPayload},
    infrastructure::{
        containers::ContainerCleanupHandler,
        messaging::{DomainEventPublisherAdapter, InMemoryEventBus},
    },
};

use crate::common::fakes::fake_runtime::FakeRuntime;

#[test]
fn publish_workflow_run_completed_reaches_bound_handler_and_cleans_up_containers() {
    let runtime = Arc::new(FakeRuntime::new());
    let cleanup_handler = Box::new(ContainerCleanupHandler::new(runtime.clone()));
    let event_bus = Arc::new(InMemoryEventBus::new(vec![cleanup_handler]));
    let publisher = DomainEventPublisherAdapter::new(event_bus);

    let event = Event::WorkflowRunCompleted(WorkflowRunCompletedPayload::new(
        "run-1".to_string(),
        "/repo".to_string(),
        vec!["container-a".to_string(), "container-b".to_string()],
        true,
    ));

    publisher.publish(event);

    assert_eq!(
        *runtime.stopped_containers.lock(),
        vec!["container-a".to_string(), "container-b".to_string()]
    );
    assert_eq!(
        *runtime.killed_containers.lock(),
        vec!["container-a".to_string(), "container-b".to_string()]
    );
    assert_eq!(
        *runtime.removed_containers.lock(),
        vec!["container-a".to_string(), "container-b".to_string()]
    );
}
