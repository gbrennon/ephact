#[derive(Debug)]
pub enum PrepareJobContainerError {
    Image(String),
    Container(String),
    Repository(String),
}

impl_application_error!(
    PrepareJobContainerError,
    |error: &PrepareJobContainerError| match error {
        PrepareJobContainerError::Image(message)
        | PrepareJobContainerError::Container(message)
        | PrepareJobContainerError::Repository(message) => message.clone(),
    },
);

impl std::error::Error for PrepareJobContainerError {}
