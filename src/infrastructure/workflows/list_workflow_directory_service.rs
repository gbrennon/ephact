use super::list_workflow_directory_port::ListWorkflowDirectoryPort;
use std::{error::Error, fs::read_dir};

use crate::application::dtos::{ListWorkflowDirectoryRequest, ListWorkflowDirectoryResponse};

/// Service that lists the workflow files held directly by one directory.
pub struct ListWorkflowDirectoryService;

impl ListWorkflowDirectoryService {
    pub fn new() -> Self {
        Self
    }

    fn is_workflow_file(path: &std::path::Path) -> bool {
        matches!(
            path.extension().and_then(|ext| ext.to_str()),
            Some("yml") | Some("yaml")
        )
    }
}

impl Default for ListWorkflowDirectoryService {
    fn default() -> Self {
        Self::new()
    }
}

impl ListWorkflowDirectoryPort for ListWorkflowDirectoryService {
    fn execute(
        &self,
        request: ListWorkflowDirectoryRequest<'_>,
    ) -> Result<ListWorkflowDirectoryResponse, Box<dyn Error>> {
        let entries = read_dir(request.directory())?;
        let mut files = Vec::new();
        for entry in entries.flatten() {
            let path = entry.path();
            if Self::is_workflow_file(&path) {
                files.push(path);
            }
        }
        files.sort();
        Ok(ListWorkflowDirectoryResponse::new(files))
    }
}
