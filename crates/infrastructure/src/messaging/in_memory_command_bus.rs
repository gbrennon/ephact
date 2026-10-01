use crate::application::ports::outbound::{
    Command, CommandError, CommandHandlerPort, CommandResponse,
};

pub struct InMemoryCommandBus {
    handler: Box<dyn CommandHandlerPort>,
}

impl InMemoryCommandBus {
    pub fn new(handler: Box<dyn CommandHandlerPort>) -> Self {
        Self { handler }
    }

    pub fn handle(&self, command: Command) -> Result<CommandResponse, CommandError> {
        self.handler.handle(command)
    }
}
