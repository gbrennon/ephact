use std::io;

#[derive(Debug)]
pub enum CopyRepositoryToContainerError {
    Filesystem(io::Error),
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
