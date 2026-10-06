use crate::{domain::aggregates::Workflow, errors::LoadWorkflowError};

/// Loads workflow content into the domain workflow representation.
pub trait WorkflowLoaderPort: Send + Sync {
    /// Parses workflow content and associates it with its source filename.
    ///
    /// # Errors
    ///
    /// Returns [`LoadWorkflowError`] when the workflow content cannot be parsed.
    fn load(&self, workflow_content: &str, file_name: &str) -> Result<Workflow, LoadWorkflowError>;
}
