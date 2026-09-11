use crate::application::dtos::responses::ShowProjectBrandingInfoResponse;

pub trait ShowProjectBrandingInfoPort {
    fn execute(&self) -> Result<ShowProjectBrandingInfoResponse, Box<dyn std::error::Error>>;
}
