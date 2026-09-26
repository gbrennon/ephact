use crate::{
    domain::entities::project_branding::ProjectBranding, errors::ProjectBrandingStoreError,
};

pub trait ProjectBrandingStorePort: Send + Sync {
    fn read_project_branding(&self) -> Result<ProjectBranding, ProjectBrandingStoreError>;
}
