#[derive(Debug)]
pub enum ProjectBrandingStoreError {
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
