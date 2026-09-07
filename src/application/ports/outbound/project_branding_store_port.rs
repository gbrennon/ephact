use crate::domain::entities::project_branding::ProjectBranding;

pub trait ProjectBrandingStorePort: Send + Sync {
    fn read_project_branding(&self) -> Result<ProjectBranding, Box<dyn std::error::Error>>;
}
