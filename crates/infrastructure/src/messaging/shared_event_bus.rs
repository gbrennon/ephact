use std::sync::Arc;

use crate::{
    application::ports::outbound::domain_event_bus_port::DomainEventBusPort,
    domain::messages::events::DomainEvent, messaging::in_memory_event_bus::InMemoryEventBus,
};

#[derive(Clone)]
pub struct SharedEventBus {
    inner: Arc<InMemoryEventBus>,
}

impl SharedEventBus {
    pub fn new(inner: Arc<InMemoryEventBus>) -> Self {
        Self { inner }
    }
}

impl DomainEventBusPort for SharedEventBus {
    fn publish(&self, event: DomainEvent) {
        self.inner.publish(event);
    }
}
