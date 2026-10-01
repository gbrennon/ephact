use crate::{
    domain::messages::commands::ExecuteWorkflowCommand, dtos::responses::WorkflowExecutionResponse,
    errors::ExecuteWorkflowError,
};

/// Handles a workflow command routed by the infrastructure command transport.
pub trait WorkflowCommandHandlerPort: Send + Sync {
    fn handle(
        &self,
        command: ExecuteWorkflowCommand,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError>;
}
