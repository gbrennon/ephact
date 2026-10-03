use std::sync::{Arc, OnceLock};

use crate::{
    application::{
        dtos::responses::{
            ExecuteActionResponse, ExecutedStepResponse, JobExecutionResponse,
            WorkflowExecutionResponse,
        },
        errors::{ExecuteJobError, ExecuteWorkflowError},
        ports::outbound::container_port::ContainerPort,
    },
    domain::{
        errors::StepError,
        messages::commands::{
            ExecuteActionPayload, ExecuteJobPayload, ExecuteStepPayload, ExecuteWorkflowPayload,
        },
    },
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

    fn bound(&self) -> Result<&InMemoryCommandBus, String> {
        self.bus
            .get()
            .ok_or_else(|| "command bus used before it was bound".to_owned())
    }

    pub fn route_workflow(
        &self,
        command: ExecuteWorkflowPayload,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError> {
        self.bound()
            .map_err(ExecuteWorkflowError::Workflow)
            .and_then(|bus| bus.handle_workflow(command))
    }

    pub fn route_job(
        &self,
        command: ExecuteJobPayload,
    ) -> Result<JobExecutionResponse, ExecuteJobError> {
        self.bound()
            .map_err(ExecuteJobError::Preparation)
            .and_then(|bus| bus.handle_job(command))
    }

    pub fn route_step(
        &self,
        command: ExecuteStepPayload,
        container: Arc<dyn ContainerPort>,
    ) -> Result<ExecutedStepResponse, StepError> {
        self.bound()
            .map_err(StepError::new)
            .and_then(|bus| bus.handle_step(command, container))
    }

    pub fn route_action(
        &self,
        command: ExecuteActionPayload,
        container: Arc<dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError> {
        self.bound()
            .map_err(StepError::new)
            .and_then(|bus| bus.handle_action(command, container))
    }
}
