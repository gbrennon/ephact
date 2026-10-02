use std::sync::Arc;

use crate::{
    application::ports::outbound::{Command, CommandError, CommandPublisherPort, CommandResponse},
    messaging::deferred_command_bus::DeferredCommandBus,
};

#[derive(Clone)]
pub struct CommandPublisherAdapter {
    inner: Arc<DeferredCommandBus>,
}

impl CommandPublisherAdapter {
    pub fn new(inner: Arc<DeferredCommandBus>) -> Self {
        Self { inner }
    }
}

impl CommandPublisherPort for CommandPublisherAdapter {
    fn publish(&self, command: Command) -> Result<CommandResponse, CommandError> {
        self.inner.route(command)
    }
}
