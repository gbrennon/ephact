use std::sync::Mutex;

use ephact_infrastructure::{
    application::ports::outbound::DomainEventPublisherPort, domain::messages::events::Event,
};

#[derive(Default)]
pub struct RecordingEvents(pub Mutex<Vec<Event>>);

impl DomainEventPublisherPort for RecordingEvents {
    fn publish(&self, event: Event) {
        self.0.lock().unwrap().push(event);
    }
}
