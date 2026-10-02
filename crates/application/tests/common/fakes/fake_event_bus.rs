use std::sync::Arc;

use ephact::{
    application::ports::outbound::domain_event_publisher_port::DomainEventPublisherPort,
    domain::messages::commands::Event,
};
use parking_lot::Mutex;

#[derive(Clone, Default)]
pub struct FakeEventBus {
    pub published_events: Arc<Mutex<Vec<Event>>>,
}

impl FakeEventBus {
    pub fn new() -> Self {
        Self {
            published_events: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn events(&self) -> Vec<Event> {
        self.published_events.lock().clone()
    }
}

impl DomainEventPublisherPort for FakeEventBus {
    fn publish(&self, event: Event) {
        self.published_events.lock().push(event);
    }
}
