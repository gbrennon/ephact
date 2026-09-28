#[derive(Debug)]
pub enum ListActionsError {
    WorkflowSource(String),
}

impl_application_error!(ListActionsError, |error: &ListActionsError| match error {
    ListActionsError::WorkflowSource(message) => format!("listing actions failed: {message}"),
},);

impl std::error::Error for ListActionsError {}
