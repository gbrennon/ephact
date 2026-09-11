use std::sync::Arc;

use crate::{
    application::ports::outbound::event_bus_port::EventBusPort,
    domain::messages::events::DomainEvent,
    infrastructure::messaging::in_memory_event_bus::InMemoryEventBus,
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

impl EventBusPort<DomainEvent> for SharedEventBus {
    fn publish(&self, event: DomainEvent) {
        self.inner.publish(event);
    }
}
