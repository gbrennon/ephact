/// Failure while listing actions available from repository workflows.
///
#[derive(Debug)]
pub enum ListActionsError {
    /// Repository metadata validation or workflow-source access failed while listing actions.
    WorkflowSource(String),
}

impl_application_error!(ListActionsError, |error: &ListActionsError| match error {
    ListActionsError::WorkflowSource(message) => format!("listing actions failed: {message}"),
},);

impl std::error::Error for ListActionsError {}
