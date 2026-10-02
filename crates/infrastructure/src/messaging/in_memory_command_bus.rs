use crate::{
    application::ports::outbound::{
        ActionCommandHandlerPort, JobCommandHandlerPort, StepCommandHandlerPort,
        WorkflowCommandHandlerPort, container_port::ContainerPort,
    },
    domain::messages::commands::{
        ExecuteActionCommand, ExecuteJobCommand, ExecuteStepCommand, ExecuteWorkflowCommand,
    },
};

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
    ) -> Result<
        crate::application::dtos::responses::WorkflowExecutionResponse,
        crate::application::errors::ExecuteWorkflowError,
    > {
        self.workflow_handler.handle(command)
    }

    pub fn handle_job(
        &self,
        command: ExecuteJobCommand,
    ) -> Result<
        crate::application::dtos::responses::JobExecutionResponse,
        crate::application::errors::ExecuteJobError,
    > {
        self.job_handler.handle(command)
    }

    pub fn handle_step(
        &self,
        command: ExecuteStepCommand<dyn ContainerPort>,
    ) -> Result<
        crate::application::dtos::responses::ExecutedStepResponse,
        crate::domain::errors::StepError,
    > {
        self.step_handler.handle(command)
    }

    pub fn handle_action(
        &self,
        command: ExecuteActionCommand<dyn ContainerPort>,
    ) -> Result<
        crate::application::dtos::responses::ExecuteActionResponse,
        crate::domain::errors::StepError,
    > {
        self.action_handler.handle(command)
    }
}
