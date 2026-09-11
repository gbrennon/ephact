use crate::domain::messages::events::{DomainEvent, Event};

/// Outbound port representing a generic event bus.
///
/// Publishes domain events to interested subscribers.
pub trait EventBusPort<E: Event>: Send + Sync {
    /// Publishes an event.
    fn publish(&self, event: E);
}

pub type DomainEventBusPort = dyn EventBusPort<DomainEvent>;
