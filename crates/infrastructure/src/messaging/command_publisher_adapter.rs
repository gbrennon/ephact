use std::sync::Arc;

use crate::{
    application::{
        dtos::responses::{
            ExecuteActionResponse, ExecutedStepResponse, JobExecutionResponse,
            WorkflowExecutionResponse,
        },
        errors::{ExecuteJobError, ExecuteWorkflowError},
        ports::outbound::{
            ActionCommandPublisherPort, JobCommandPublisherPort, StepCommandPublisherPort,
            WorkflowCommandPublisherPort, container_port::ContainerPort,
        },
    },
    domain::{
        errors::StepError,
        messages::commands::{
            ExecuteActionCommand, ExecuteJobCommand, ExecuteStepCommand, ExecuteWorkflowCommand,
        },
    },
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

impl WorkflowCommandPublisherPort for CommandPublisherAdapter {
    fn publish(
        &self,
        command: ExecuteWorkflowCommand,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError> {
        self.inner.route_workflow(command)
    }
}

impl JobCommandPublisherPort for CommandPublisherAdapter {
    fn publish(&self, command: ExecuteJobCommand) -> Result<JobExecutionResponse, ExecuteJobError> {
        self.inner.route_job(command)
    }
}

impl StepCommandPublisherPort for CommandPublisherAdapter {
    fn publish(
        &self,
        command: ExecuteStepCommand<dyn ContainerPort>,
    ) -> Result<ExecutedStepResponse, StepError> {
        self.inner.route_step(command)
    }
}

impl ActionCommandPublisherPort for CommandPublisherAdapter {
    fn publish(
        &self,
        command: ExecuteActionCommand<dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError> {
        self.inner.route_action(command)
    }
}
