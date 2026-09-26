#[derive(Debug, thiserror::Error)]
pub enum ProjectBrandingStoreError {
    #[error("project branding could not be read: {0}")]
    Read(String),
}
