use crate::domain::messages::events::DomainEvent;

/// Publishes domain events to interested subscribers.
pub trait DomainEventBusPort: Send + Sync {
    /// Publishes a domain event to interested subscribers.
    fn publish(&self, event: DomainEvent);
}
