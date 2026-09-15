use std::sync::OnceLock;

use crate::{
    application::{
        dtos::responses::{
            ExecuteActionResponse, ExecutedStepResponse, JobExecutionResponse,
            WorkflowExecutionResponse,
        },
        errors::{ExecuteJobError, ExecuteWorkflowError},
        ports::outbound::{
            action_command_bus_port::ActionCommandBusPort, container_port::ContainerPort,
            job_command_bus_port::JobCommandBusPort, step_command_bus_port::StepCommandBusPort,
            workflow_command_bus_port::WorkflowCommandBusPort,
        },
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
impl WorkflowCommandBusPort for DeferredCommandBus {
    fn dispatch(
        &self,
        command: ExecuteWorkflowCommand,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError> {
        let bus = self
            .bound()
            .ok_or_else(|| ExecuteWorkflowError::Workflow(Self::unbound().message().to_string()))?;
        WorkflowCommandBusPort::dispatch(bus, command)
            .map_err(|error| ExecuteWorkflowError::Workflow(error.to_string()))
    }
}

impl JobCommandBusPort for DeferredCommandBus {
    fn dispatch(
        &self,
        command: ExecuteJobCommand,
    ) -> Result<JobExecutionResponse, ExecuteJobError> {
        let bus = self
            .bound()
            .ok_or_else(|| ExecuteJobError::Preparation(Self::unbound().message().to_string()))?;
        JobCommandBusPort::dispatch(bus, command)
            .map_err(|error| ExecuteJobError::Preparation(error.to_string()))
    }
}

impl StepCommandBusPort for DeferredCommandBus {
    fn dispatch<'a>(
        &self,
        command: ExecuteStepCommand<'a, dyn ContainerPort>,
    ) -> Result<ExecutedStepResponse, StepError> {
        let bus = self.bound().ok_or_else(Self::unbound)?;
        StepCommandBusPort::dispatch(bus, command)
    }
}

impl ActionCommandBusPort for DeferredCommandBus {
    fn dispatch<'a>(
        &self,
        command: ExecuteActionCommand<'a, dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError> {
        let bus = self.bound().ok_or_else(Self::unbound)?;
        ActionCommandBusPort::dispatch(bus, command)
    }
}
