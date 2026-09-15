use crate::application::dtos::responses::ShowProjectBrandingInfoResponse;
use crate::application::errors::ShowProjectBrandingInfoError;

pub trait ShowProjectBrandingInfoPort {
    fn execute(&self) -> Result<ShowProjectBrandingInfoResponse, ShowProjectBrandingInfoError>;
}
