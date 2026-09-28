use crate::{domain::errors::ProjectBrandingError, errors::ProjectBrandingStoreError};

#[derive(Debug)]
pub enum ShowProjectBrandingInfoError {
    Branding(ProjectBrandingError),
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
