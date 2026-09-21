use crate::application::{
    dtos::responses::ShowProjectBrandingInfoResponse,
    errors::ShowProjectBrandingInfoError,
    ports::{inbound::ShowProjectBrandingInfoPort, outbound::ProjectBrandingStorePort},
};

pub struct ShowProjectBrandingInfoService {
    branding_store: Box<dyn ProjectBrandingStorePort>,
}

impl ShowProjectBrandingInfoService {
    pub fn new(branding_store: Box<dyn ProjectBrandingStorePort>) -> Self {
        Self { branding_store }
    }
}

impl ShowProjectBrandingInfoPort for ShowProjectBrandingInfoService {
    fn execute(&self) -> Result<ShowProjectBrandingInfoResponse, ShowProjectBrandingInfoError> {
        let branding = self
            .branding_store
            .read_project_branding()
            .map_err(ShowProjectBrandingInfoError::Store)?;

        Ok(ShowProjectBrandingInfoResponse::new(
            branding.name().as_str().to_string(),
            branding.description().as_str().to_string(),
            branding.version().as_str().to_string(),
            branding.emblem().as_str().to_string(),
        ))
    }
}
