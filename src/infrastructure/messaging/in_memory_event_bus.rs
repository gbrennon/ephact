use super::domain_event_handler::DomainEventHandler;
use crate::{
    application::ports::outbound::event_bus_port::EventBusPort,
    domain::messages::events::DomainEvent,
};

/// Event bus that dispatches published domain events to the in-process
/// handlers interested in them, in registration order.
pub struct InMemoryEventBus {
    handlers: Vec<Box<dyn DomainEventHandler>>,
}

impl InMemoryEventBus {
    pub fn new(handlers: Vec<Box<dyn DomainEventHandler>>) -> Self {
        Self { handlers }
    }
}

impl EventBusPort<DomainEvent> for InMemoryEventBus {
    fn publish(&self, event: DomainEvent) {
        for handler in &self.handlers {
            handler.handle(&event);
        }
    }
}
