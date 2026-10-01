use crate::dtos::{requests::ResolveWorkflowFilesRequest, responses::ResolveWorkflowFilesResponse};

pub trait ResolveWorkflowFilesPort: Send + Sync {
    fn resolve(
        &self,
        request: ResolveWorkflowFilesRequest,
    ) -> Result<ResolveWorkflowFilesResponse, Box<dyn std::error::Error>>;
}
