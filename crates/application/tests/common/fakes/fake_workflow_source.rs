use std::sync::Arc;

use ephact::{
    application::{
        dtos::responses::{WorkflowListItemResponse, WorkflowSourceFileResponse},
        errors::WorkflowSourceError,
        ports::outbound::WorkflowSourcePort,
    },
    domain::Repository,
};
use parking_lot::Mutex;

use super::fake_workflow_source_state::{
    FakeWorkflowSourceState, FakeWorkflowSourceStateOperation, FakeWorkflowSourceStateResult,
};

type RecordedWorkflowCall = (Repository, Option<String>);

/// Canned [`WorkflowSourcePort`] keeping its state behind `Arc<Mutex<_>>` so the
/// port stays `Send + Sync`, and so a clone held by the test can still observe
/// the calls made through the boxed clone owned by the service.
#[derive(Clone, Default)]
pub struct FakeWorkflowSource {
    state: Arc<Mutex<FakeWorkflowSourceState>>,
    read_workflow_calls: Arc<Mutex<Vec<RecordedWorkflowCall>>>,
    read_all_workflows_calls: Arc<Mutex<Vec<Repository>>>,
    list_actions_calls: Arc<Mutex<Vec<Repository>>>,
    list_workflows_calls: Arc<Mutex<Vec<Repository>>>,
}

impl FakeWorkflowSource {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_workflows(self, workflows: Vec<WorkflowListItemResponse>) -> Self {
        self.state.lock().configure_workflows(workflows);
        self
    }

    pub fn with_actions(self, actions: Vec<String>) -> Self {
        self.state.lock().configure_actions(actions);
        self
    }

    pub fn with_workflow_content(self, content: &str) -> Self {
        self.state
            .lock()
            .configure_content(Some(content), None, None);
        self
    }

    pub fn with_workflow_file_name(self, file_name: &str) -> Self {
        self.state
            .lock()
            .configure_content(None, Some(file_name), None);
        self
    }

    pub fn with_all_workflow_contents(self, contents: Vec<String>) -> Self {
        self.state
            .lock()
            .configure_content(None, None, Some(contents));
        self
    }

    pub fn failing_read_workflow(self, message: &str) -> Self {
        self.state
            .lock()
            .configure_failures(Some(message), None, None, None);
        self
    }

    pub fn failing_read_all_workflows(self, message: &str) -> Self {
        self.state
            .lock()
            .configure_failures(None, Some(message), None, None);
        self
    }

    pub fn failing_list_actions(self, message: &str) -> Self {
        self.state
            .lock()
            .configure_failures(None, None, Some(message), None);
        self
    }

    pub fn failing_list_workflows(self, message: &str) -> Self {
        self.state
            .lock()
            .configure_failures(None, None, None, Some(message));
        self
    }

    pub fn read_workflow_calls(&self) -> Vec<(Repository, Option<String>)> {
        self.read_workflow_calls.lock().clone()
    }

    pub fn read_all_workflows_calls(&self) -> Vec<Repository> {
        self.read_all_workflows_calls.lock().clone()
    }

    pub fn list_actions_calls(&self) -> Vec<Repository> {
        self.list_actions_calls.lock().clone()
    }

    pub fn list_workflows_calls(&self) -> Vec<Repository> {
        self.list_workflows_calls.lock().clone()
    }
}

impl WorkflowSourcePort for FakeWorkflowSource {
    fn read_workflow(
        &self,
        repository: &Repository,
        workflow_name: Option<&str>,
    ) -> Result<WorkflowSourceFileResponse, WorkflowSourceError> {
        self.read_workflow_calls
            .lock()
            .push((repository.clone(), workflow_name.map(str::to_string)));

        let operation = FakeWorkflowSourceStateOperation::ReadWorkflow {
            repository: repository.clone(),
            workflow_name: workflow_name.map(str::to_string),
        };
        let result = self.state.lock().execute(operation);
        let FakeWorkflowSourceStateResult::Workflow(result) = result else {
            unreachable!();
        };
        result
    }

    fn read_workflow_in_directory(
        &self,
        repository: &Repository,
        workflow_name: Option<&str>,
        _workflow_directory: Option<&std::path::Path>,
    ) -> Result<WorkflowSourceFileResponse, WorkflowSourceError> {
        self.read_workflow(repository, workflow_name)
    }

    fn read_all_workflows(
        &self,
        repository: &Repository,
    ) -> Result<Vec<WorkflowSourceFileResponse>, WorkflowSourceError> {
        self.read_all_workflows_calls
            .lock()
            .push(repository.clone());

        let operation = FakeWorkflowSourceStateOperation::ReadAllWorkflows {
            repository: repository.clone(),
        };
        let result = self.state.lock().execute(operation);
        let FakeWorkflowSourceStateResult::AllWorkflows(result) = result else {
            unreachable!();
        };
        result
    }

    fn read_all_workflows_in_directory(
        &self,
        repository: &Repository,
        _workflow_directory: Option<&std::path::Path>,
    ) -> Result<Vec<WorkflowSourceFileResponse>, WorkflowSourceError> {
        self.read_all_workflows(repository)
    }

    fn list_actions(&self, repository: &Repository) -> Result<Vec<String>, WorkflowSourceError> {
        self.list_actions_calls.lock().push(repository.clone());

        let operation = FakeWorkflowSourceStateOperation::ListActions {
            repository: repository.clone(),
        };
        let result = self.state.lock().execute(operation);
        let FakeWorkflowSourceStateResult::Actions(result) = result else {
            unreachable!();
        };
        result
    }

    fn list_workflows(
        &self,
        repository: &Repository,
    ) -> Result<Vec<WorkflowListItemResponse>, WorkflowSourceError> {
        self.list_workflows_calls.lock().push(repository.clone());

        let operation = FakeWorkflowSourceStateOperation::ListWorkflows {
            repository: repository.clone(),
        };
        let result = self.state.lock().execute(operation);
        let FakeWorkflowSourceStateResult::Workflows(result) = result else {
            unreachable!();
        };
        result
    }
}
