use std::{error::Error, sync::OnceLock};

use crate::{
    application::{
        dtos::responses::{
            ExecuteActionResponse, ExecutedStepResponse, JobExecutionResponse,
            WorkflowExecutionResponse,
        },
        ports::outbound::{command_bus_port::CommandBusPort, container_port::ContainerPort},
    },
    domain::{
        errors::StepError,
        messages::commands::{
            ExecuteActionCommand, ExecuteJobCommand, ExecuteStepCommand, ExecuteWorkflowCommand,
        },
    },
    infrastructure::messaging::in_memory_command_bus::InMemoryCommandBus,
};

/// Command bus whose target is bound after construction.
///
/// The command graph is cyclic by design: a coordination service publishes a
/// command that a handler turns into a call on the next coordination service.
/// Handing every service this proxy first, then binding the assembled bus,
/// closes that cycle without any service holding a handler.
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

    /// Binds the bus every dispatch is forwarded to.
    ///
    /// # Panics
    ///
    /// Panics when a bus is already bound: rebinding would silently reroute
    /// commands already in flight.
    pub fn bind(&self, bus: InMemoryCommandBus) {
        assert!(self.bus.set(bus).is_ok(), "command bus already bound");
    }

    fn bound(&self) -> Option<&InMemoryCommandBus> {
        self.bus.get()
    }

    fn unbound() -> StepError {
        StepError::new("command bus used before it was bound".to_string())
    }
}

impl CommandBusPort<ExecuteWorkflowCommand> for DeferredCommandBus {
    type Response = WorkflowExecutionResponse;
    type Error = Box<dyn Error>;

    fn dispatch(&self, command: ExecuteWorkflowCommand) -> Result<Self::Response, Self::Error> {
        self.bound()
            .ok_or_else(|| -> Box<dyn Error> { Self::unbound().message().to_string().into() })?
            .dispatch(command)
    }
}

impl CommandBusPort<ExecuteJobCommand> for DeferredCommandBus {
    type Response = JobExecutionResponse;
    type Error = Box<dyn Error>;

    fn dispatch(&self, command: ExecuteJobCommand) -> Result<Self::Response, Self::Error> {
        self.bound()
            .ok_or_else(|| -> Box<dyn Error> { Self::unbound().message().to_string().into() })?
            .dispatch(command)
    }
}

impl<'a> CommandBusPort<ExecuteStepCommand<'a, dyn ContainerPort>> for DeferredCommandBus {
    type Response = ExecutedStepResponse;
    type Error = StepError;

    fn dispatch(
        &self,
        command: ExecuteStepCommand<'a, dyn ContainerPort>,
    ) -> Result<Self::Response, Self::Error> {
        self.bound().ok_or_else(Self::unbound)?.dispatch(command)
    }
}

impl<'a> CommandBusPort<ExecuteActionCommand<'a, dyn ContainerPort>> for DeferredCommandBus {
    type Response = ExecuteActionResponse;
    type Error = StepError;

    fn dispatch(
        &self,
        command: ExecuteActionCommand<'a, dyn ContainerPort>,
    ) -> Result<Self::Response, Self::Error> {
        self.bound().ok_or_else(Self::unbound)?.dispatch(command)
    }
}
