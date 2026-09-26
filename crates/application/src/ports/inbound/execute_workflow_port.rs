use crate::{
    dtos::{requests::ExecuteWorkflowRequest, responses::WorkflowExecutionResponse},
    errors::ExecuteWorkflowError,
};

pub trait ExecuteWorkflowPort: Send + Sync {
    fn execute(
        &self,
        request: ExecuteWorkflowRequest,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError>;
}
