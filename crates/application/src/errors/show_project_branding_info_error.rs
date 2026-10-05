use crate::{domain::errors::ProjectBrandingError, errors::ProjectBrandingStoreError};

/// Failure while loading project branding for display.
///
#[derive(Debug)]
pub enum ShowProjectBrandingInfoError {
    /// Project branding failed domain validation.
    Branding(ProjectBrandingError),
    /// The branding store could not provide project branding.
    Store(ProjectBrandingStoreError),
}

impl_application_error!(
    ShowProjectBrandingInfoError,
    |error: &ShowProjectBrandingInfoError| match error {
        ShowProjectBrandingInfoError::Branding(error) =>
            format!("project branding validation failed: {error}"),
        ShowProjectBrandingInfoError::Store(error) =>
            format!("project branding store failed: {error}"),
    },
);

impl std::error::Error for ShowProjectBrandingInfoError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Branding(error) => Some(error),
            Self::Store(error) => Some(error),
        }
    }
}
