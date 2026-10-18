use ephact::{
    application::{
        dtos::responses::{WorkflowListItemResponse, WorkflowSourceFileResponse},
        errors::WorkflowSourceError,
    },
    domain::Repository,
};

#[derive(Default)]
pub struct FakeWorkflowSourceState {
    workflows: Vec<WorkflowListItemResponse>,
    actions: Vec<String>,
    workflow_content: String,
    workflow_file_name: String,
    all_workflow_contents: Vec<String>,
    read_workflow_error: Option<String>,
    read_all_workflows_error: Option<String>,
    list_actions_error: Option<String>,
    list_workflows_error: Option<String>,
}

pub enum FakeWorkflowSourceStateOperation {
    ReadWorkflow {
        repository: Repository,
        workflow_name: Option<String>,
    },
    ReadAllWorkflows {
        repository: Repository,
    },
    ListActions {
        repository: Repository,
    },
    ListWorkflows {
        repository: Repository,
    },
}

pub enum FakeWorkflowSourceStateResult {
    Workflow(Result<WorkflowSourceFileResponse, WorkflowSourceError>),
    AllWorkflows(Result<Vec<WorkflowSourceFileResponse>, WorkflowSourceError>),
    Actions(Result<Vec<String>, WorkflowSourceError>),
    Workflows(Result<Vec<WorkflowListItemResponse>, WorkflowSourceError>),
}

impl FakeWorkflowSourceState {
    pub fn configure_workflows(&mut self, workflows: Vec<WorkflowListItemResponse>) {
        self.workflows = workflows;
    }

    pub fn configure_actions(&mut self, actions: Vec<String>) {
        self.actions = actions;
    }

    pub fn configure_content(
        &mut self,
        content: Option<&str>,
        file_name: Option<&str>,
        all_contents: Option<Vec<String>>,
    ) {
        if let Some(content) = content {
            self.workflow_content = content.to_string();
        }
        if let Some(file_name) = file_name {
            self.workflow_file_name = file_name.to_string();
        }
        if let Some(all_contents) = all_contents {
            self.all_workflow_contents = all_contents;
        }
    }

    pub fn configure_failures(
        &mut self,
        read_workflow: Option<&str>,
        read_all_workflows: Option<&str>,
        list_actions: Option<&str>,
        list_workflows: Option<&str>,
    ) {
        if let Some(message) = read_workflow {
            self.read_workflow_error = Some(message.to_string());
        }
        if let Some(message) = read_all_workflows {
            self.read_all_workflows_error = Some(message.to_string());
        }
        if let Some(message) = list_actions {
            self.list_actions_error = Some(message.to_string());
        }
        if let Some(message) = list_workflows {
            self.list_workflows_error = Some(message.to_string());
        }
    }

    pub fn execute(
        &mut self,
        operation: FakeWorkflowSourceStateOperation,
    ) -> FakeWorkflowSourceStateResult {
        match operation {
            FakeWorkflowSourceStateOperation::ReadWorkflow {
                repository,
                workflow_name,
            } => FakeWorkflowSourceStateResult::Workflow(
                self.read_workflow(&repository, workflow_name.as_deref()),
            ),
            FakeWorkflowSourceStateOperation::ReadAllWorkflows { repository } => {
                FakeWorkflowSourceStateResult::AllWorkflows(self.read_all_workflows(&repository))
            }
            FakeWorkflowSourceStateOperation::ListActions { repository } => {
                FakeWorkflowSourceStateResult::Actions(self.list_actions(&repository))
            }
            FakeWorkflowSourceStateOperation::ListWorkflows { repository } => {
                FakeWorkflowSourceStateResult::Workflows(self.list_workflows(&repository))
            }
        }
    }

    fn read_workflow(
        &self,
        repository: &Repository,
        workflow_name: Option<&str>,
    ) -> Result<WorkflowSourceFileResponse, WorkflowSourceError> {
        if let Some(message) = &self.read_workflow_error {
            return Err(WorkflowSourceError::NotFound(message.clone()));
        }

        let _ = (repository, workflow_name);
        Ok(WorkflowSourceFileResponse::new(
            self.workflow_content.clone(),
            self.workflow_file_name.clone(),
        ))
    }

    fn read_all_workflows(
        &self,
        repository: &Repository,
    ) -> Result<Vec<WorkflowSourceFileResponse>, WorkflowSourceError> {
        if let Some(message) = &self.read_all_workflows_error {
            return Err(WorkflowSourceError::NotFound(message.clone()));
        }

        let _ = repository;
        Ok(self
            .all_workflow_contents
            .iter()
            .enumerate()
            .map(|(index, content)| {
                WorkflowSourceFileResponse::new(content.clone(), format!("workflow-{index}.yml"))
            })
            .collect())
    }

    fn list_actions(&self, repository: &Repository) -> Result<Vec<String>, WorkflowSourceError> {
        if let Some(message) = &self.list_actions_error {
            return Err(WorkflowSourceError::NotFound(message.clone()));
        }

        let _ = repository;
        Ok(self.actions.clone())
    }

    fn list_workflows(
        &self,
        repository: &Repository,
    ) -> Result<Vec<WorkflowListItemResponse>, WorkflowSourceError> {
        if let Some(message) = &self.list_workflows_error {
            return Err(WorkflowSourceError::NotFound(message.clone()));
        }

        let _ = repository;
        Ok(self.workflows.clone())
    }
}
