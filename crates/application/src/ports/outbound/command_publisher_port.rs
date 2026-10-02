use super::{Command, CommandError, CommandResponse};

pub trait CommandPublisherPort: Send + Sync {
    fn publish(&self, command: Command) -> Result<CommandResponse, CommandError>;
}
