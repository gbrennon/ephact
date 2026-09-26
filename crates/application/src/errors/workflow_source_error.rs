use std::io;

#[derive(Debug, thiserror::Error)]
pub enum WorkflowSourceError {
    #[error("{0}")]
    Io(#[source] io::Error),
    #[error("{0}")]
    NotFound(String),
    #[error("no workflow files found")]
    Empty,
}
