use crate::{
    dtos::responses::ShowProjectBrandingInfoResponse, errors::ShowProjectBrandingInfoError,
};

/// Provides the project's display branding information.
pub trait ShowProjectBrandingInfoPort {
    /// Reads and returns the project's name, description, version, and emblem.
    ///
    /// # Errors
    ///
    /// Returns [`ShowProjectBrandingInfoError`] when branding cannot be read or
    /// validated.
    fn execute(&self) -> Result<ShowProjectBrandingInfoResponse, ShowProjectBrandingInfoError>;
}
