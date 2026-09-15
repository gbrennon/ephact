use crate::domain::messages::events::DomainEvent;

/// Outbound port publishing domain events to interested subscribers.
///
/// Publishes a [`DomainEvent`] to every subscriber interested in it.
pub trait DomainEventBusPort: Send + Sync {
    /// Publishes a domain event.
    fn publish(&self, event: DomainEvent);
}
