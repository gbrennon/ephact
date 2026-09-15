use std::sync::Arc;

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
    infrastructure::messaging::deferred_command_bus::DeferredCommandBus,
};

#[derive(Clone)]
pub struct SharedCommandBus {
    inner: Arc<DeferredCommandBus>,
}

impl SharedCommandBus {
    pub fn new(inner: Arc<DeferredCommandBus>) -> Self {
        Self { inner }
    }
}

impl WorkflowCommandBusPort for SharedCommandBus {
    fn dispatch(
        &self,
        command: ExecuteWorkflowCommand,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError> {
        WorkflowCommandBusPort::dispatch(&*self.inner, command)
    }
}

impl JobCommandBusPort for SharedCommandBus {
    fn dispatch(
        &self,
        command: ExecuteJobCommand,
    ) -> Result<JobExecutionResponse, ExecuteJobError> {
        JobCommandBusPort::dispatch(&*self.inner, command)
    }
}

impl StepCommandBusPort for SharedCommandBus {
    fn dispatch<'a>(
        &self,
        command: ExecuteStepCommand<'a, dyn ContainerPort>,
    ) -> Result<ExecutedStepResponse, StepError> {
        StepCommandBusPort::dispatch(&*self.inner, command)
    }
}

impl ActionCommandBusPort for SharedCommandBus {
    fn dispatch<'a>(
        &self,
        command: ExecuteActionCommand<'a, dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError> {
        ActionCommandBusPort::dispatch(&*self.inner, command)
    }
}
