/// Failure while preparing a job's execution container.
///
#[derive(Debug)]
pub enum PrepareJobContainerError {
    /// The job's container image could not be pulled.
    Image(String),
    /// The job's container could not be created.
    Container(String),
    /// The repository could not be copied into the job container.
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
