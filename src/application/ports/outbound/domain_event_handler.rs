use crate::domain::events::DomainEvent;

/// Outbound port for handlers subscribed to domain events.
///
/// Implementors react to the variants they are interested in and ignore the
/// rest. The event bus dispatches every published event to every registered
/// handler, in registration order.
pub trait DomainEventHandler {
    /// Handles one published domain event.
    fn handle(&self, event: &DomainEvent);
}
