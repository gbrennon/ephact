use crate::{
    domain::messages::commands::ExecuteWorkflowPayload, dtos::responses::WorkflowExecutionResponse,
    errors::ExecuteWorkflowError,
};

pub trait WorkflowCommandHandlerPort: Send + Sync {
    fn handle(
        &self,
        command: ExecuteWorkflowPayload,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError>;
}
