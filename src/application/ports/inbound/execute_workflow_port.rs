use crate::application::dtos::requests::ExecuteWorkflowRequest;
use crate::application::dtos::responses::WorkflowExecutionResponse;
use crate::application::errors::ExecuteWorkflowError;

pub trait ExecuteWorkflowPort: Send + Sync {
    fn execute(
        &self,
        request: ExecuteWorkflowRequest,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError>;
}
