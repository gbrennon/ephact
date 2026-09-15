use crate::application::dtos::responses::WorkflowListItemResponse;
use crate::application::errors::WorkflowSourceError;
use crate::domain::entities::repository::Repository;

pub trait WorkflowSourcePort: Send + Sync {
    fn read_workflow(
        &self,
        repository: &Repository,
        workflow_name: Option<&str>,
    ) -> Result<String, WorkflowSourceError>;

    fn read_all_workflows(
        &self,
        repository: &Repository,
    ) -> Result<Vec<String>, WorkflowSourceError>;

    fn list_actions(&self, repository: &Repository) -> Result<Vec<String>, WorkflowSourceError>;

    fn list_workflows(
        &self,
        repository: &Repository,
    ) -> Result<Vec<WorkflowListItemResponse>, WorkflowSourceError>;
}
