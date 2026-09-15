#![allow(dead_code)]
use parking_lot::Mutex;
use std::sync::Arc;

use ephact::{
    application::ports::outbound::domain_event_bus_port::DomainEventBusPort,
    domain::messages::events::DomainEvent,
};

#[derive(Clone, Default)]
pub struct FakeEventBus {
    pub published_events: Arc<Mutex<Vec<DomainEvent>>>,
}

impl FakeEventBus {
    pub fn new() -> Self {
        Self {
            published_events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn events(&self) -> Vec<DomainEvent> {
        self.published_events.lock().clone()
    }
}

impl DomainEventBusPort for FakeEventBus {
    fn publish(&self, event: DomainEvent) {
        self.published_events.lock().push(event);
    }
}
