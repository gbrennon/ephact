use crate::{
    application::ports::outbound::domain_event_handler_port::DomainEventHandlerPort,
    domain::messages::events::Event,
};

/// Infrastructure event transport that fans every published event out to the
/// bound handlers in registration order. It is an internal bus detail: the
/// application never depends on it, reaching it only through the
/// [`DomainEventPublisherAdapter`](super::DomainEventPublisherAdapter).
pub struct InMemoryEventBus {
    handlers: Vec<Box<dyn DomainEventHandlerPort>>,
}

impl InMemoryEventBus {
    pub fn new(handlers: Vec<Box<dyn DomainEventHandlerPort>>) -> Self {
        Self { handlers }
    }

    pub fn publish(&self, event: Event) {
        for handler in &self.handlers {
            handler.handle(&event);
        }
    }
}
