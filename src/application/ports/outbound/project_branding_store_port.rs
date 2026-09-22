use crate::{
    application::errors::ProjectBrandingStoreError,
    domain::entities::project_branding::ProjectBranding,
};

pub trait ProjectBrandingStorePort: Send + Sync {
    fn read_project_branding(&self) -> Result<ProjectBranding, ProjectBrandingStoreError>;
}
