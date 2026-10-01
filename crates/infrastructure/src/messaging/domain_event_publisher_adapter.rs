use std::sync::Arc;

use crate::{
    application::ports::outbound::domain_event_publisher_port::DomainEventPublisherPort,
    domain::messages::events::DomainEvent, messaging::in_memory_event_bus::InMemoryEventBus,
};

/// Routes the application's [`DomainEventPublisherPort`] onto the infrastructure
/// [`InMemoryEventBus`]. This is the only application-facing event publisher;
/// the bus itself stays an internal detail.
#[derive(Clone)]
pub struct DomainEventPublisherAdapter {
    inner: Arc<InMemoryEventBus>,
}

impl DomainEventPublisherAdapter {
    pub fn new(inner: Arc<InMemoryEventBus>) -> Self {
        Self { inner }
    }
}

impl DomainEventPublisherPort for DomainEventPublisherAdapter {
    fn publish(&self, event: DomainEvent) {
        self.inner.publish(event);
    }
}
