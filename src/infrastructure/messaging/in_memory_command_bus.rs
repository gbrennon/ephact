use std::error::Error;

use crate::application::commands::{
    ExecuteActionCommand, ExecuteJobCommand, ExecuteStepCommand, ExecuteWorkflowCommand,
};
use crate::application::dtos::responses::ExecuteActionResponse;
use crate::application::dtos::responses::ExecutedStepResponse;
use crate::application::dtos::responses::JobExecutionResponse;
use crate::application::dtos::responses::WorkflowExecutionResponse;
use crate::application::ports::outbound::CommandBusPort;
use crate::domain::errors::StepError;
use crate::infrastructure::actions::ActionCommandHandler;
use crate::infrastructure::jobs::JobCommandHandler;
use crate::infrastructure::steps::StepCommandHandler;
use crate::infrastructure::workflows::WorkflowCommandHandler;

/// In-memory implementation of the CommandBusPort.
///
/// Routes commands (intentions of something to happen in the future)
/// to their corresponding infrastructure command handlers.
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

impl CommandBusPort for InMemoryCommandBus {
    fn dispatch_workflow(
        &self,
        cmd: ExecuteWorkflowCommand,
    ) -> Result<WorkflowExecutionResponse, Box<dyn Error>> {
        self.workflow_handler.handle(cmd)
    }

    fn dispatch_job(&self, cmd: ExecuteJobCommand) -> Result<JobExecutionResponse, Box<dyn Error>> {
        self.job_handler.handle(cmd)
    }

    fn dispatch_step(&self, cmd: ExecuteStepCommand) -> Result<ExecutedStepResponse, StepError> {
        self.step_handler.handle(cmd)
    }

    fn dispatch_action(
        &self,
        cmd: ExecuteActionCommand,
    ) -> Result<ExecuteActionResponse, StepError> {
        self.action_handler.handle(cmd)
    }
}
