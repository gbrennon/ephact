use super::{Command, CommandError, CommandResponse};

pub trait CommandHandlerPort: Send + Sync {
    fn handle(&self, command: Command) -> Result<CommandResponse, CommandError>;
}
