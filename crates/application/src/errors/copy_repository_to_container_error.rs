use std::io;

/// Failure while collecting repository files or copying them into a container.
///
#[derive(Debug)]
pub enum CopyRepositoryToContainerError {
    /// A filesystem operation needed to collect repository files failed.
    Filesystem(io::Error),
    /// A container copy operation failed after the repository files were collected.
    Container(String),
}

impl_application_error!(
    CopyRepositoryToContainerError,
    |error: &CopyRepositoryToContainerError| match error {
        CopyRepositoryToContainerError::Filesystem(error) => error.to_string(),
        CopyRepositoryToContainerError::Container(message) => message.clone(),
    },
);

impl std::error::Error for CopyRepositoryToContainerError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Filesystem(error) => Some(error),
            Self::Container(_) => None,
        }
    }
}
