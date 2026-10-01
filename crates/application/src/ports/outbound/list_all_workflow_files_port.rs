use crate::dtos::{requests::ListAllWorkflowFilesRequest, responses::ListAllWorkflowFilesResponse};

pub trait ListAllWorkflowFilesPort: Send + Sync {
    fn list(
        &self,
        request: ListAllWorkflowFilesRequest,
    ) -> Result<ListAllWorkflowFilesResponse, Box<dyn std::error::Error>>;
}
