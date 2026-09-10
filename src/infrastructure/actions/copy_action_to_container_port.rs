use crate::application::dtos::requests::CopyActionToContainerRequest;
use crate::domain::errors::StepError;

/// Inbound port for copying an action's files into the job's container.
pub trait CopyActionToContainerPort: Send + Sync {
    /// Copies the action in and returns the container-side directory.
    fn execute(&self, request: CopyActionToContainerRequest<'_>) -> Result<String, StepError>;
}
