use std::sync::OnceLock;

use crate::{
    application::ports::outbound::{Command, CommandError, CommandResponse},
    messaging::in_memory_command_bus::InMemoryCommandBus,
};

#[derive(Default)]
pub struct DeferredCommandBus {
    bus: OnceLock<InMemoryCommandBus>,
}

impl DeferredCommandBus {
    #[must_use]
    pub fn new() -> Self {
        Self {
            bus: OnceLock::new(),
        }
    }

    pub fn bind(&self, bus: InMemoryCommandBus) {
        assert!(self.bus.set(bus).is_ok(), "command bus already bound");
    }

    fn bound(&self) -> Option<&InMemoryCommandBus> {
        self.bus.get()
    }

    pub fn route(&self, command: Command) -> Result<CommandResponse, CommandError> {
        self.bound()
            .ok_or_else(|| {
                CommandError::Transport("command bus used before it was bound".to_string())
            })?
            .handle(command)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::domain::{aggregates::Workflow, entities::Job, value_objects::EvaluationContext};

    #[test]
    fn route_reports_unbound_transport_without_misclassifying_command() {
        let bus = DeferredCommandBus::new();
        let command = Command::Job(Box::new(
            crate::domain::messages::commands::ExecuteJobCommand::new(
                Job::default(),
                "job".to_string(),
                Workflow::new(None, Vec::new(), Default::default(), Default::default()),
                PathBuf::from("/repo"),
                EvaluationContext::new(),
            ),
        ));

        let error = bus.route(command).unwrap_err();

        assert!(
            matches!(error, CommandError::Transport(message) if message == "command bus used before it was bound")
        );
    }
}
