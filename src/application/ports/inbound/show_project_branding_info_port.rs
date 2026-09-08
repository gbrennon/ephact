use crate::application::dtos::ShowProjectBrandingInfoResponse;

pub trait ShowProjectBrandingInfoPort {
    fn execute(&self) -> Result<ShowProjectBrandingInfoResponse, Box<dyn std::error::Error>>;
}
