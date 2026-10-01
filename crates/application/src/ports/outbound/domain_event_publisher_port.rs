use crate::domain::messages::events::DomainEvent;

/// Publishes domain events to whichever infrastructure transport is bound.
///
/// Application services depend on this outbound port rather than on any
/// concrete event bus, so the bus stays an infrastructure detail.
pub trait DomainEventPublisherPort: Send + Sync {
    fn publish(&self, event: DomainEvent);
}
