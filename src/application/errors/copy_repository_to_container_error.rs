use std::io;

#[derive(Debug, thiserror::Error)]
pub enum CopyRepositoryToContainerError {
    #[error("{0}")]
    Filesystem(#[source] io::Error),
    #[error("{0}")]
    Container(String),
}
