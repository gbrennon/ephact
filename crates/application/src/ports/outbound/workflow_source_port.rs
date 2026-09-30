use crate::{
    domain::entities::repository::Repository,
    dtos::responses::{WorkflowListItemResponse, WorkflowSourceFileResponse},
    errors::WorkflowSourceError,
};

pub trait WorkflowSourcePort: Send + Sync {
    fn read_workflow(
        &self,
        repository: &Repository,
        workflow_name: Option<&str>,
    ) -> Result<WorkflowSourceFileResponse, WorkflowSourceError>;

    fn read_all_workflows(
        &self,
        repository: &Repository,
    ) -> Result<Vec<WorkflowSourceFileResponse>, WorkflowSourceError>;

    fn list_actions(&self, repository: &Repository) -> Result<Vec<String>, WorkflowSourceError>;

    fn list_workflows(
        &self,
        repository: &Repository,
    ) -> Result<Vec<WorkflowListItemResponse>, WorkflowSourceError>;
}
