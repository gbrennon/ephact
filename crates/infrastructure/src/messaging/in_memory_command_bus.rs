use crate::{
    application::{
        dtos::responses::{
            ExecuteActionResponse, ExecutedStepResponse, JobExecutionResponse,
            WorkflowExecutionResponse,
        },
        errors::{ExecuteJobError, ExecuteWorkflowError},
        ports::outbound::{
            ActionCommandHandlerPort, JobCommandHandlerPort, StepCommandHandlerPort,
            WorkflowCommandHandlerPort, container_port::ContainerPort,
        },
    },
    domain::{
        errors::StepError,
        messages::commands::{
            ExecuteActionCommand, ExecuteJobCommand, ExecuteStepCommand, ExecuteWorkflowCommand,
        },
    },
};

/// Infrastructure command transport that routes each command kind to the
/// handler bound for it. The handlers implement the application's command
/// handler ports, so the bus is a pure routing detail that no application code
/// depends on.
pub struct InMemoryCommandBus {
    workflow_handler: Box<dyn WorkflowCommandHandlerPort>,
    job_handler: Box<dyn JobCommandHandlerPort>,
    step_handler: Box<dyn StepCommandHandlerPort>,
    action_handler: Box<dyn ActionCommandHandlerPort>,
}

impl InMemoryCommandBus {
    pub fn new(
        workflow_handler: Box<dyn WorkflowCommandHandlerPort>,
        job_handler: Box<dyn JobCommandHandlerPort>,
        step_handler: Box<dyn StepCommandHandlerPort>,
        action_handler: Box<dyn ActionCommandHandlerPort>,
    ) -> Self {
        Self {
            workflow_handler,
            job_handler,
            step_handler,
            action_handler,
        }
    }

    pub fn handle_workflow(
        &self,
        command: ExecuteWorkflowCommand,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError> {
        self.workflow_handler.handle(command)
    }

    pub fn handle_job(
        &self,
        command: ExecuteJobCommand,
    ) -> Result<JobExecutionResponse, ExecuteJobError> {
        self.job_handler.handle(command)
    }

    pub fn handle_step(
        &self,
        command: ExecuteStepCommand<dyn ContainerPort>,
    ) -> Result<ExecutedStepResponse, StepError> {
        self.step_handler.handle(command)
    }

    pub fn handle_action(
        &self,
        command: ExecuteActionCommand<dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError> {
        self.action_handler.handle(command)
    }
}
