#[derive(Debug, thiserror::Error)]
pub enum PrepareJobContainerError {
    #[error("{0}")]
    Image(String),
    #[error("{0}")]
    Container(String),
    #[error("{0}")]
    Repository(String),
}
