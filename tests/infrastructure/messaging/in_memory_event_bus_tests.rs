use std::sync::Arc;

use ephact::{
    application::ports::outbound::EventBusPort,
    domain::events::{ActRunCompletedPayload, DomainEvent},
    infrastructure::{containers::ContainerCleanupHandler, messaging::InMemoryEventBus},
};

use crate::common::fakes::fake_runtime::FakeRuntime;

#[test]
fn publish_act_run_completed_stops_kills_and_removes_containers() {
    let runtime = Arc::new(FakeRuntime::new());
    let cleanup_handler = Box::new(ContainerCleanupHandler::new(runtime.clone()));
    let bus = InMemoryEventBus::new(vec![cleanup_handler]);

    let event = DomainEvent::ActRunCompleted(ActRunCompletedPayload::new(
        vec!["container-a".to_string(), "container-b".to_string()],
        true,
    ));

    bus.publish(event);

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
