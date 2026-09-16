use crate::application::dtos::requests::BuildRunContextRequest;
use crate::application::dtos::responses::BuildRunContextResponse;

/// Inbound port for building the expression context a run is evaluated against.
pub trait BuildRunContextPort: Send + Sync {
    fn execute(&self, request: BuildRunContextRequest) -> BuildRunContextResponse;
}
