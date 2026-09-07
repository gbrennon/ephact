use crate::{
    application::ports::outbound::ProjectBrandingStorePort,
    domain::{ProjectBranding, ProjectDescription, ProjectEmblem, ProjectName, ProjectVersion},
};

pub struct CargoProjectBrandingStore;

impl ProjectBrandingStorePort for CargoProjectBrandingStore {
    fn read_project_branding(&self) -> Result<ProjectBranding, Box<dyn std::error::Error>> {
        Ok(ProjectBranding::new(
            ProjectName::new(env!("CARGO_PKG_NAME").to_string())?,
            ProjectDescription::new(env!("CARGO_PKG_DESCRIPTION").to_string())?,
            ProjectVersion::new(env!("CARGO_PKG_VERSION").to_string())?,
            ProjectEmblem::new(
                include_str!("../../assets/project_emblem.txt")
                    .trim_end()
                    .to_string(),
            )?,
        ))
    }
}
