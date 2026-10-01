use crate::domain::messages::events::DomainEvent;

/// Handles a domain event delivered by the bound infrastructure transport.
///
/// Event subscribers (container cleanup, failure logging, progress reporting)
/// implement this application-owned port so the transport can route events to
/// them without knowing their concrete types.
pub trait DomainEventHandlerPort: Send + Sync {
    fn handle(&self, event: &DomainEvent);
}
