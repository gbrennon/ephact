use crate::application::dtos::requests::ResolveActionDirectoryRequest;
use crate::application::dtos::responses::ResolvedActionDirectoryResponse;
use crate::domain::errors::StepError;

/// Inbound port for deciding where the action a step references lives.
pub trait ResolveActionDirectoryPort: Send + Sync {
    /// Classifies the reference and resolves it to a directory.
    fn execute(
        &self,
        request: ResolveActionDirectoryRequest<'_>,
    ) -> Result<ResolvedActionDirectoryResponse, StepError>;
}
