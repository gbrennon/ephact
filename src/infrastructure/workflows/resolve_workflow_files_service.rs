use super::{
    detect_workflow_file_port::DetectWorkflowFilePort,
    list_all_workflow_files_port::ListAllWorkflowFilesPort,
    resolve_named_workflow_file_port::ResolveNamedWorkflowFilePort,
    resolve_workflow_files_port::ResolveWorkflowFilesPort,
};
use std::error::Error;

use crate::application::dtos::{
    DetectWorkflowFileRequest, ListAllWorkflowFilesRequest, ResolveNamedWorkflowFileRequest,
    ResolveWorkflowFilesRequest, ResolveWorkflowFilesResponse,
};

/// Service that decides which workflow files a run executes: every workflow of
/// the repository, the one the caller named, or the detected default.
pub struct ResolveWorkflowFilesService {
    all_lister: Box<dyn ListAllWorkflowFilesPort>,
    named_resolver: Box<dyn ResolveNamedWorkflowFilePort>,
    detector: Box<dyn DetectWorkflowFilePort>,
}

impl ResolveWorkflowFilesService {
    pub fn new(
        all_lister: Box<dyn ListAllWorkflowFilesPort>,
        named_resolver: Box<dyn ResolveNamedWorkflowFilePort>,
        detector: Box<dyn DetectWorkflowFilePort>,
    ) -> Self {
        Self {
            all_lister,
            named_resolver,
            detector,
        }
    }

    fn resolve_all(
        &self,
        repo_path: &std::path::Path,
    ) -> Result<Vec<std::path::PathBuf>, Box<dyn Error>> {
        let response = self
            .all_lister
            .execute(ListAllWorkflowFilesRequest::new(repo_path))?;
        Ok(response.workflow_files().to_vec().to_vec())
    }

    fn resolve_named(
        &self,
        workflow: &str,
        repo_path: &std::path::Path,
    ) -> Result<Vec<std::path::PathBuf>, Box<dyn Error>> {
        let file = self
            .named_resolver
            .execute(ResolveNamedWorkflowFileRequest::new(workflow, repo_path))?;
        Ok(vec![file])
    }

    fn resolve_detected(
        &self,
        repo_path: &std::path::Path,
    ) -> Result<Vec<std::path::PathBuf>, Box<dyn Error>> {
        let file = self
            .detector
            .execute(DetectWorkflowFileRequest::new(repo_path))?;
        Ok(vec![file])
    }

    fn resolve_files(
        &self,
        request: &ResolveWorkflowFilesRequest<'_>,
    ) -> Result<Vec<std::path::PathBuf>, Box<dyn Error>> {
        if request.config().all_workflows() {
            return self.resolve_all(request.repo_path());
        }
        if let Some(workflow) = request.config().workflow() {
            return self.resolve_named(workflow.as_str(), request.repo_path());
        }
        self.resolve_detected(request.repo_path())
    }
}

impl ResolveWorkflowFilesPort for ResolveWorkflowFilesService {
    fn execute(
        &self,
        request: ResolveWorkflowFilesRequest<'_>,
    ) -> Result<ResolveWorkflowFilesResponse, Box<dyn Error>> {
        let workflow_files = self.resolve_files(&request)?;
        Ok(ResolveWorkflowFilesResponse::new(workflow_files))
    }
}
