use std::path::PathBuf;

use crate::dtos::requests::ResolveNamedWorkflowFileRequest;

pub trait ResolveNamedWorkflowFilePort: Send + Sync {
    fn resolve(
        &self,
        request: ResolveNamedWorkflowFileRequest,
    ) -> Result<PathBuf, Box<dyn std::error::Error>>;
}
