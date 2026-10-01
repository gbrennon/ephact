use std::error::Error;

use crate::application::{
    dtos::{
        requests::{
            DetectWorkflowFileRequest, ListAllWorkflowFilesRequest,
            ResolveNamedWorkflowFileRequest, ResolveWorkflowFilesRequest,
        },
        responses::ResolveWorkflowFilesResponse,
    },
    ports::outbound::{
        DetectWorkflowFilePort, ListAllWorkflowFilesPort, ResolveNamedWorkflowFilePort,
        ResolveWorkflowFilesPort,
    },
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
            .list(ListAllWorkflowFilesRequest::new(repo_path.to_path_buf()))?;
        Ok(response.workflow_files().to_vec().to_vec())
    }

    fn resolve_named(
        &self,
        workflow: &str,
        repo_path: &std::path::Path,
    ) -> Result<Vec<std::path::PathBuf>, Box<dyn Error>> {
        let file = self
            .named_resolver
            .resolve(ResolveNamedWorkflowFileRequest::new(
                workflow.to_string(),
                repo_path.to_path_buf(),
            ))?;
        Ok(vec![file])
    }

    fn resolve_detected(
        &self,
        repo_path: &std::path::Path,
    ) -> Result<Vec<std::path::PathBuf>, Box<dyn Error>> {
        let file = self
            .detector
            .detect(DetectWorkflowFileRequest::new(repo_path.to_path_buf()))?;
        Ok(vec![file])
    }

    fn resolve_files(
        &self,
        request: &ResolveWorkflowFilesRequest,
    ) -> Result<Vec<std::path::PathBuf>, Box<dyn Error>> {
        match (
            request.config().all_workflows(),
            request.config().workflow(),
        ) {
            (true, _) => self.resolve_all(request.repo_path()),
            (false, Some(workflow)) => self.resolve_named(workflow.as_str(), request.repo_path()),
            (false, None) => self.resolve_detected(request.repo_path()),
        }
    }
}

impl ResolveWorkflowFilesPort for ResolveWorkflowFilesService {
    fn resolve(
        &self,
        request: ResolveWorkflowFilesRequest,
    ) -> Result<ResolveWorkflowFilesResponse, Box<dyn Error>> {
        let workflow_files = self.resolve_files(&request)?;
        Ok(ResolveWorkflowFilesResponse::new(workflow_files))
    }
}
