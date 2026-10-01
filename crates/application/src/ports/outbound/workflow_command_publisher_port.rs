use crate::{
    domain::messages::commands::ExecuteWorkflowCommand, dtos::responses::WorkflowExecutionResponse,
    errors::ExecuteWorkflowError,
};

/// Publishes a workflow command to the bound infrastructure command transport.
pub trait WorkflowCommandPublisherPort: Send + Sync {
    fn publish(
        &self,
        command: ExecuteWorkflowCommand,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError>;
}
