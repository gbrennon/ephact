use crate::dtos::{
    requests::ListWorkflowDirectoryRequest, responses::ListWorkflowDirectoryResponse,
};

pub trait ListWorkflowDirectoryPort: Send + Sync {
    fn list(
        &self,
        request: ListWorkflowDirectoryRequest,
    ) -> Result<ListWorkflowDirectoryResponse, Box<dyn std::error::Error>>;
}
