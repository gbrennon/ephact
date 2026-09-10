use crate::application::dtos::requests::ExecuteWorkflowRequest;
use crate::application::dtos::responses::WorkflowExecutionResponse;

pub trait ExecuteWorkflowPort: Send + Sync {
    fn execute(
        &self,
        request: ExecuteWorkflowRequest<'_>,
    ) -> Result<WorkflowExecutionResponse, Box<dyn std::error::Error>>;
}
