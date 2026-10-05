/// Failure while reading project branding from its store.
///
#[derive(Debug)]
pub enum ProjectBrandingStoreError {
    /// The store could not read or validate project branding.
    Read(String),
}

impl_application_error!(
    ProjectBrandingStoreError,
    |error: &ProjectBrandingStoreError| match error {
        ProjectBrandingStoreError::Read(message) =>
            format!("project branding could not be read: {message}"),
    },
);

impl std::error::Error for ProjectBrandingStoreError {}
