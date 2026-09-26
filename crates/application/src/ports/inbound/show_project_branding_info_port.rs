use crate::{
    dtos::responses::ShowProjectBrandingInfoResponse, errors::ShowProjectBrandingInfoError,
};

pub trait ShowProjectBrandingInfoPort {
    fn execute(&self) -> Result<ShowProjectBrandingInfoResponse, ShowProjectBrandingInfoError>;
}
