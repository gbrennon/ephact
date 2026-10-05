use crate::{
    domain::entities::repository::Repository,
    dtos::responses::{WorkflowListItemResponse, WorkflowSourceFileResponse},
    errors::WorkflowSourceError,
};

/// Reads and indexes workflow source files for a repository.
pub trait WorkflowSourcePort: Send + Sync {
    /// Reads the first workflow file when `workflow_name` is `None`, or the
    /// workflow whose resolved name matches the supplied name.
    ///
    /// # Errors
    ///
    /// Returns [`WorkflowSourceError`] when files cannot be read, no workflow
    /// exists, or the named workflow is not found.
    fn read_workflow(
        &self,
        repository: &Repository,
        workflow_name: Option<&str>,
    ) -> Result<WorkflowSourceFileResponse, WorkflowSourceError>;

    /// Reads every workflow source file in the repository's supported workflow
    /// directories.
    ///
    /// # Errors
    ///
    /// Returns [`WorkflowSourceError`] when a workflow file cannot be read.
    fn read_all_workflows(
        &self,
        repository: &Repository,
    ) -> Result<Vec<WorkflowSourceFileResponse>, WorkflowSourceError>;

    /// Returns the unique action references found in workflow `uses` entries.
    ///
    /// # Errors
    ///
    /// Returns [`WorkflowSourceError`] when a workflow file cannot be read.
    fn list_actions(&self, repository: &Repository) -> Result<Vec<String>, WorkflowSourceError>;

    /// Returns one item per workflow file with its resolved name, file path, and
    /// declared trigger events.
    ///
    /// # Errors
    ///
    /// Returns [`WorkflowSourceError`] when workflow files cannot be read.
    fn list_workflows(
        &self,
        repository: &Repository,
    ) -> Result<Vec<WorkflowListItemResponse>, WorkflowSourceError>;
}
