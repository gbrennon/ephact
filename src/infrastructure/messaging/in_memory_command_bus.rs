use std::error::Error;

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
    infrastructure::{
        actions::ActionCommandHandler, jobs::JobCommandHandler, steps::StepCommandHandler,
        workflows::WorkflowCommandHandler,
    },
};

/// In-memory implementation of the CommandBusPort.
///
/// Routes commands to their corresponding infrastructure command handlers.
pub struct InMemoryCommandBus {
    workflow_handler: Box<WorkflowCommandHandler>,
    job_handler: Box<JobCommandHandler>,
    step_handler: Box<StepCommandHandler>,
    action_handler: Box<ActionCommandHandler>,
}

impl InMemoryCommandBus {
    pub fn new(
        workflow_handler: Box<WorkflowCommandHandler>,
        job_handler: Box<JobCommandHandler>,
        step_handler: Box<StepCommandHandler>,
        action_handler: Box<ActionCommandHandler>,
    ) -> Self {
        Self {
            workflow_handler,
            job_handler,
            step_handler,
            action_handler,
        }
    }
}

impl CommandBusPort<ExecuteWorkflowCommand> for InMemoryCommandBus {
    type Response = WorkflowExecutionResponse;
    type Error = Box<dyn Error>;

    fn dispatch(&self, command: ExecuteWorkflowCommand) -> Result<Self::Response, Self::Error> {
        self.workflow_handler.handle(command)
    }
}

impl CommandBusPort<ExecuteJobCommand> for InMemoryCommandBus {
    type Response = JobExecutionResponse;
    type Error = Box<dyn Error>;

    fn dispatch(&self, command: ExecuteJobCommand) -> Result<Self::Response, Self::Error> {
        self.job_handler.handle(command)
    }
}

impl<'a> CommandBusPort<ExecuteStepCommand<'a, dyn ContainerPort>> for InMemoryCommandBus {
    type Response = ExecutedStepResponse;
    type Error = StepError;

    fn dispatch(
        &self,
        command: ExecuteStepCommand<'a, dyn ContainerPort>,
    ) -> Result<Self::Response, Self::Error> {
        self.step_handler.handle(command)
    }
}

impl<'a> CommandBusPort<ExecuteActionCommand<'a, dyn ContainerPort>> for InMemoryCommandBus {
    type Response = ExecuteActionResponse;
    type Error = StepError;

    fn dispatch(
        &self,
        command: ExecuteActionCommand<'a, dyn ContainerPort>,
    ) -> Result<Self::Response, Self::Error> {
        self.action_handler.handle(command)
    }
}
