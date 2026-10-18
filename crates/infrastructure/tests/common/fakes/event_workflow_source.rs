use std::path::Path;

use ephact::{
    application::{
        dtos::responses::{WorkflowListItemResponse, WorkflowSourceFileResponse},
        errors::WorkflowSourceError,
        ports::outbound::WorkflowSourcePort,
    },
    domain::Repository,
};

use super::fake_workflow_source::FakeWorkflowSource;

pub struct EventWorkflowSource {
    source: FakeWorkflowSource,
    payload: String,
}

impl EventWorkflowSource {
    pub fn new(payload: &str) -> Self {
        Self {
            source: FakeWorkflowSource::new(),
            payload: payload.to_owned(),
        }
    }
}

impl WorkflowSourcePort for EventWorkflowSource {
    fn read_workflow(
        &self,
        repository: &Repository,
        workflow_name: Option<&str>,
    ) -> Result<WorkflowSourceFileResponse, WorkflowSourceError> {
        self.source.read_workflow(repository, workflow_name)
    }

    fn read_workflow_in_directory(
        &self,
        repository: &Repository,
        workflow_name: Option<&str>,
        workflow_directory: Option<&Path>,
    ) -> Result<WorkflowSourceFileResponse, WorkflowSourceError> {
        self.source
            .read_workflow_in_directory(repository, workflow_name, workflow_directory)
    }

    fn read_all_workflows(
        &self,
        repository: &Repository,
    ) -> Result<Vec<WorkflowSourceFileResponse>, WorkflowSourceError> {
        self.source.read_all_workflows(repository)
    }

    fn read_all_workflows_in_directory(
        &self,
        repository: &Repository,
        workflow_directory: Option<&Path>,
    ) -> Result<Vec<WorkflowSourceFileResponse>, WorkflowSourceError> {
        self.source
            .read_all_workflows_in_directory(repository, workflow_directory)
    }

    fn read_file(&self, _path: &Path) -> Result<String, WorkflowSourceError> {
        Ok(self.payload.clone())
    }

    fn list_actions(&self, repository: &Repository) -> Result<Vec<String>, WorkflowSourceError> {
        self.source.list_actions(repository)
    }

    fn list_workflows(
        &self,
        repository: &Repository,
    ) -> Result<Vec<WorkflowListItemResponse>, WorkflowSourceError> {
        self.source.list_workflows(repository)
    }
}
