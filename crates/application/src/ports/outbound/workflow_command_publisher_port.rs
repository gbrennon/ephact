use crate::{
    domain::messages::commands::ExecuteWorkflowPayload, dtos::responses::WorkflowExecutionResponse,
    errors::ExecuteWorkflowError,
};

pub trait WorkflowCommandPublisherPort: Send + Sync {
    fn publish(
        &self,
        command: ExecuteWorkflowPayload,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError>;
}
