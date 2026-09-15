use crate::domain::messages::events::DomainEvent;

/// Outbound port for publishing a domain event.
///
/// Implementations deliver a [`DomainEvent`] to every interested subscriber.
pub trait DomainEventBusPort: Send + Sync {
    /// Publishes a domain event to interested subscribers.
    fn publish(&self, event: DomainEvent);
}
