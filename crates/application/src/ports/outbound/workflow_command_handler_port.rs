use crate::{
    domain::messages::commands::ExecuteWorkflowCommand, dtos::responses::WorkflowExecutionResponse,
    errors::ExecuteWorkflowError,
};

pub trait WorkflowCommandHandlerPort: Send + Sync {
    fn handle(
        &self,
        command: ExecuteWorkflowCommand,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError>;
}
