use ephact::{
    application::{
        dtos::responses::{WorkflowListItemResponse, WorkflowSourceFileResponse},
        errors::WorkflowSourceError,
    },
    domain::Repository,
};

#[derive(Clone, Default)]
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
    pub fn configure_workflows(self, workflows: Vec<WorkflowListItemResponse>) -> Self {
        Self { workflows, ..self }
    }

    pub fn configure_actions(self, actions: Vec<String>) -> Self {
        Self { actions, ..self }
    }

    pub fn configure_content(
        self,
        content: Option<&str>,
        file_name: Option<&str>,
        all_contents: Option<Vec<String>>,
    ) -> Self {
        Self {
            workflow_content: content.map(str::to_owned).unwrap_or(self.workflow_content),
            workflow_file_name: file_name
                .map(str::to_owned)
                .unwrap_or(self.workflow_file_name),
            all_workflow_contents: all_contents.unwrap_or(self.all_workflow_contents),
            ..self
        }
    }

    pub fn configure_failures(
        self,
        read_workflow: Option<&str>,
        read_all_workflows: Option<&str>,
        list_actions: Option<&str>,
        list_workflows: Option<&str>,
    ) -> Self {
        Self {
            read_workflow_error: read_workflow
                .map(str::to_owned)
                .or(self.read_workflow_error),
            read_all_workflows_error: read_all_workflows
                .map(str::to_owned)
                .or(self.read_all_workflows_error),
            list_actions_error: list_actions.map(str::to_owned).or(self.list_actions_error),
            list_workflows_error: list_workflows
                .map(str::to_owned)
                .or(self.list_workflows_error),
            ..self
        }
    }

    pub fn execute(
        &self,
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
#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use ephact::domain::{RepoPath, RepositoryName};

    use super::*;

    fn repository() -> Repository {
        Repository::new(
            RepoPath::new(PathBuf::from("workspace")).expect("valid repository path"),
            RepositoryName::new("test-repo".to_string()).expect("valid repository name"),
        )
    }

    #[test]
    fn immutable_configuration_drives_each_execution_result() {
        let repository = repository();
        let state = FakeWorkflowSourceState::default()
            .configure_workflows(vec![WorkflowListItemResponse::new(
                Some("build".to_string()),
                Some("build.yml".to_string()),
                vec!["push".to_string()],
            )])
            .configure_actions(vec!["build".to_string()])
            .configure_content(
                Some("name: Build"),
                Some("build.yml"),
                Some(vec!["name: Build".to_string()]),
            );

        let FakeWorkflowSourceStateResult::Workflows(Ok(workflows)) =
            state.execute(FakeWorkflowSourceStateOperation::ListWorkflows {
                repository: repository.clone(),
            })
        else {
            panic!("expected workflow list result");
        };
        assert_eq!(workflows.len(), 1);
        assert_eq!(workflows[0].name(), Some("build"));

        let FakeWorkflowSourceStateResult::Actions(Ok(actions)) =
            state.execute(FakeWorkflowSourceStateOperation::ListActions {
                repository: repository.clone(),
            })
        else {
            panic!("expected action list result");
        };
        assert_eq!(actions, vec!["build".to_string()]);

        let FakeWorkflowSourceStateResult::Workflow(Ok(workflow)) =
            state.execute(FakeWorkflowSourceStateOperation::ReadWorkflow {
                repository: repository.clone(),
                workflow_name: Some("build".to_string()),
            })
        else {
            panic!("expected workflow result");
        };
        assert_eq!(workflow.content(), "name: Build");
        assert_eq!(workflow.file_name(), "build.yml");

        let FakeWorkflowSourceStateResult::AllWorkflows(Ok(workflows)) =
            state.execute(FakeWorkflowSourceStateOperation::ReadAllWorkflows { repository })
        else {
            panic!("expected all-workflows result");
        };
        assert_eq!(workflows.len(), 1);
        assert_eq!(workflows[0].content(), "name: Build");
        assert_eq!(workflows[0].file_name(), "workflow-0.yml");
    }

    #[test]
    fn immutable_failure_configuration_is_returned_by_execution() {
        let repository = repository();
        let state = FakeWorkflowSourceState::default().configure_failures(
            Some("workflow missing"),
            Some("workflows missing"),
            Some("actions missing"),
            Some("workflow list missing"),
        );

        let FakeWorkflowSourceStateResult::Workflow(Err(WorkflowSourceError::NotFound(message))) =
            state.execute(FakeWorkflowSourceStateOperation::ReadWorkflow {
                repository: repository.clone(),
                workflow_name: None,
            })
        else {
            panic!("expected workflow failure");
        };
        assert_eq!(message, "workflow missing");

        let FakeWorkflowSourceStateResult::AllWorkflows(Err(WorkflowSourceError::NotFound(
            message,
        ))) = state.execute(FakeWorkflowSourceStateOperation::ReadAllWorkflows {
            repository: repository.clone(),
        })
        else {
            panic!("expected all-workflows failure");
        };
        assert_eq!(message, "workflows missing");

        let FakeWorkflowSourceStateResult::Actions(Err(WorkflowSourceError::NotFound(message))) =
            state.execute(FakeWorkflowSourceStateOperation::ListActions {
                repository: repository.clone(),
            })
        else {
            panic!("expected action failure");
        };
        assert_eq!(message, "actions missing");

        let FakeWorkflowSourceStateResult::Workflows(Err(WorkflowSourceError::NotFound(message))) =
            state.execute(FakeWorkflowSourceStateOperation::ListWorkflows { repository })
        else {
            panic!("expected workflow-list failure");
        };
        assert_eq!(message, "workflow list missing");
    }
}
