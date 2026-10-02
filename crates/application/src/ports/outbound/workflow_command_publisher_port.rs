use crate::{
    domain::messages::commands::ExecuteWorkflowCommand, dtos::responses::WorkflowExecutionResponse,
    errors::ExecuteWorkflowError,
};

pub trait WorkflowCommandPublisherPort: Send + Sync {
    fn publish(
        &self,
        command: ExecuteWorkflowCommand,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError>;
}
