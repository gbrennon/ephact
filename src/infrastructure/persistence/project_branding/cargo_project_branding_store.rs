use crate::{
    application::{errors::ProjectBrandingStoreError, ports::outbound::ProjectBrandingStorePort},
    domain::{ProjectBranding, ProjectDescription, ProjectEmblem, ProjectName, ProjectVersion},
};

/// Reads project branding metadata compiled into the binary from Cargo package variables
/// and embedded text assets.
#[derive(Debug, Clone, Copy, Default)]
pub struct CargoProjectBrandingStore {}

impl CargoProjectBrandingStore {
    /// Creates a new instance of the Cargo project branding store.
    #[must_use]
    pub fn new() -> Self {
        Self {}
    }
}

impl ProjectBrandingStorePort for CargoProjectBrandingStore {
    fn read_project_branding(&self) -> Result<ProjectBranding, ProjectBrandingStoreError> {
        Ok(ProjectBranding::new(
            ProjectName::new(env!("CARGO_PKG_NAME").to_string())
                .map_err(|error| ProjectBrandingStoreError::Read(error.to_string()))?,
            ProjectDescription::new(env!("CARGO_PKG_DESCRIPTION").to_string())
                .map_err(|error| ProjectBrandingStoreError::Read(error.to_string()))?,
            ProjectVersion::new(env!("CARGO_PKG_VERSION").to_string())
                .map_err(|error| ProjectBrandingStoreError::Read(error.to_string()))?,
            ProjectEmblem::new(
                include_str!("../../../../assets/project_emblem.txt")
                    .trim_end()
                    .to_string(),
            )
            .map_err(|error| ProjectBrandingStoreError::Read(error.to_string()))?,
        ))
    }
}
