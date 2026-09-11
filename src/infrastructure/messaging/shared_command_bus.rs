use std::{error::Error, sync::Arc};

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

impl CommandBusPort<ExecuteWorkflowCommand> for SharedCommandBus {
    type Response = WorkflowExecutionResponse;
    type Error = Box<dyn Error>;

    fn dispatch(&self, command: ExecuteWorkflowCommand) -> Result<Self::Response, Self::Error> {
        self.inner.dispatch(command)
    }
}

impl CommandBusPort<ExecuteJobCommand> for SharedCommandBus {
    type Response = JobExecutionResponse;
    type Error = Box<dyn Error>;

    fn dispatch(&self, command: ExecuteJobCommand) -> Result<Self::Response, Self::Error> {
        self.inner.dispatch(command)
    }
}

impl<'a> CommandBusPort<ExecuteStepCommand<'a, dyn ContainerPort>> for SharedCommandBus {
    type Response = ExecutedStepResponse;
    type Error = StepError;

    fn dispatch(
        &self,
        command: ExecuteStepCommand<'a, dyn ContainerPort>,
    ) -> Result<Self::Response, Self::Error> {
        self.inner.dispatch(command)
    }
}

impl<'a> CommandBusPort<ExecuteActionCommand<'a, dyn ContainerPort>> for SharedCommandBus {
    type Response = ExecuteActionResponse;
    type Error = StepError;

    fn dispatch(
        &self,
        command: ExecuteActionCommand<'a, dyn ContainerPort>,
    ) -> Result<Self::Response, Self::Error> {
        self.inner.dispatch(command)
    }
}
