use crate::application::dtos::requests::CollectActionFilesRequest;
use crate::application::dtos::responses::CollectActionFilesResponse;
use crate::domain::errors::StepError;

/// Inbound port for reading the files that make up an action.
pub trait CollectActionFilesPort: Send + Sync {
    /// Walks the action directory and reads every file it holds.
    fn execute(
        &self,
        request: CollectActionFilesRequest<'_>,
    ) -> Result<CollectActionFilesResponse, StepError>;
}
