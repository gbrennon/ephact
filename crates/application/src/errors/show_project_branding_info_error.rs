use crate::{domain::errors::ProjectBrandingError, errors::ProjectBrandingStoreError};

#[derive(Debug, thiserror::Error)]
pub enum ShowProjectBrandingInfoError {
    #[error("project branding validation failed: {0}")]
    Branding(#[source] ProjectBrandingError),
    #[error("project branding store failed: {0}")]
    Store(#[source] ProjectBrandingStoreError),
}
