use std::error::Error;

use crate::application::{
    dtos::ShowProjectBrandingInfoResponse,
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
    fn execute(&self) -> Result<ShowProjectBrandingInfoResponse, Box<dyn Error>> {
        let branding = self.branding_store.read_project_branding()?;

        Ok(ShowProjectBrandingInfoResponse {
            name: branding.name().as_str().to_string(),
            description: branding.description().as_str().to_string(),
            version: branding.version().as_str().to_string(),
            emblem: branding.emblem().as_str().to_string(),
        })
    }
}
