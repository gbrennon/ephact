use std::error::Error;

use crate::application::dtos::responses::WorkflowListItemResponse;
use crate::domain::entities::repository::Repository;

/// Outbound port that supplies workflow definitions and catalogues for a repository.
///
/// The application layer states *what* it needs (a workflow definition, the list of
/// workflows, the actions in use) and never *how* it is obtained. Storage details -
/// directory layout, file extensions, parsing - live entirely in the infrastructure
/// adapter that implements this port.
pub trait WorkflowSourcePort: Send + Sync {
    /// Reads the definition of one workflow, either named explicitly or auto-detected.
    fn read_workflow(
        &self,
        repository: &Repository,
        workflow_name: Option<&str>,
    ) -> Result<String, Box<dyn Error>>;

    /// Reads the definition of every workflow the repository declares.
    fn read_all_workflows(&self, repository: &Repository) -> Result<Vec<String>, Box<dyn Error>>;

    /// Lists the unique action references used across the repository's workflows.
    fn list_actions(&self, repository: &Repository) -> Result<Vec<String>, Box<dyn Error>>;

    /// Lists a summary item for each workflow the repository declares.
    fn list_workflows(
        &self,
        repository: &Repository,
    ) -> Result<Vec<WorkflowListItemResponse>, Box<dyn Error>>;
}
