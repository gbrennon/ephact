use crate::{
    application::{errors::ProjectBrandingStoreError, ports::outbound::ProjectBrandingStorePort},
    domain::{ProjectBranding, ProjectDescription, ProjectEmblem, ProjectName, ProjectVersion},
};

/// Reads project branding metadata compiled into the binary from Cargo package variables
/// and embedded text assets.
#[derive(Debug, Clone)]
pub struct CargoProjectBrandingStore {
    name: String,
    description: String,
    version: String,
    emblem: String,
}

impl CargoProjectBrandingStore {
    #[must_use]
    pub fn from_metadata(name: &str, description: &str, version: &str, emblem: &str) -> Self {
        Self {
            name: name.to_string(),
            description: description.to_string(),
            version: version.to_string(),
            emblem: emblem.trim_end().to_string(),
        }
    }
}

impl ProjectBrandingStorePort for CargoProjectBrandingStore {
    fn read_project_branding(&self) -> Result<ProjectBranding, ProjectBrandingStoreError> {
        Ok(ProjectBranding::new(
            ProjectName::new(self.name.clone())
                .map_err(|error| ProjectBrandingStoreError::Read(error.to_string()))?,
            ProjectDescription::new(self.description.clone())
                .map_err(|error| ProjectBrandingStoreError::Read(error.to_string()))?,
            ProjectVersion::new(self.version.clone())
                .map_err(|error| ProjectBrandingStoreError::Read(error.to_string()))?,
            ProjectEmblem::new(self.emblem.clone())
                .map_err(|error| ProjectBrandingStoreError::Read(error.to_string()))?,
        ))
    }
}
