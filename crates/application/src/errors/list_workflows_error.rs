/// Failure while listing workflows available from a repository.
///
#[derive(Debug)]
pub enum ListWorkflowsError {
    /// Repository metadata validation or workflow-source access failed while listing workflows.
    WorkflowSource(String),
}

impl_application_error!(
    ListWorkflowsError,
    |error: &ListWorkflowsError| match error {
        ListWorkflowsError::WorkflowSource(message) =>
            format!("listing workflows failed: {message}"),
    },
);

impl std::error::Error for ListWorkflowsError {}
