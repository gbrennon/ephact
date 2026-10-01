use crate::dtos::{requests::MergeRunExecutionsRequest, responses::WorkflowExecutionResponse};

pub trait MergeRunExecutionsPort: Send + Sync {
    fn merge(
        &self,
        request: MergeRunExecutionsRequest,
    ) -> Result<WorkflowExecutionResponse, Box<dyn std::error::Error>>;
}
