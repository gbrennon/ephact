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

impl WorkflowCommandBusPort for InMemoryCommandBus {
    fn dispatch(
        &self,
        command: ExecuteWorkflowCommand,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError> {
        self.workflow_handler
            .handle(command)
            .map_err(|error| ExecuteWorkflowError::Workflow(error.to_string()))
    }
}

impl JobCommandBusPort for InMemoryCommandBus {
    fn dispatch(
        &self,
        command: ExecuteJobCommand,
    ) -> Result<JobExecutionResponse, ExecuteJobError> {
        self.job_handler
            .handle(command)
            .map_err(|error| ExecuteJobError::Preparation(error.to_string()))
    }
}

impl StepCommandBusPort for InMemoryCommandBus {
    fn dispatch<'a>(
        &self,
        command: ExecuteStepCommand<'a, dyn ContainerPort>,
    ) -> Result<ExecutedStepResponse, StepError> {
        self.step_handler.handle(command)
    }
}

impl ActionCommandBusPort for InMemoryCommandBus {
    fn dispatch<'a>(
        &self,
        command: ExecuteActionCommand<'a, dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError> {
        self.action_handler.handle(command)
    }
}
