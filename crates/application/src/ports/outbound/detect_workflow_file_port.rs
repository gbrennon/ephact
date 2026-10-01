use std::path::PathBuf;

use crate::dtos::requests::DetectWorkflowFileRequest;

pub trait DetectWorkflowFilePort: Send + Sync {
    fn detect(
        &self,
        request: DetectWorkflowFileRequest,
    ) -> Result<PathBuf, Box<dyn std::error::Error>>;
}
