use crate::{
    application::ports::outbound::{DomainEventHandler, EventBusPort},
    domain::events::DomainEvent,
    infrastructure::containers::ContainerCleanupHandler,
};

/// Event bus that dispatches published domain events to the in-process
/// handlers interested in them, in registration order.
pub struct InMemoryEventBus {
    handlers: Vec<Box<dyn DomainEventHandler + Send + Sync>>,
}

impl InMemoryEventBus {
    pub fn new(handlers: Vec<Box<dyn DomainEventHandler + Send + Sync>>) -> Self {
        Self { handlers }
    }

    /// Bus with only the container cleanup handler registered.
    pub fn with_cleanup_handler(cleanup_handler: Box<ContainerCleanupHandler>) -> Self {
        Self::new(vec![cleanup_handler])
    }
}

impl EventBusPort for InMemoryEventBus {
    fn publish(&self, event: DomainEvent) {
        for handler in &self.handlers {
            handler.handle(&event);
        }
    }
}
