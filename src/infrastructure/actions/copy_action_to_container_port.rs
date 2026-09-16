use crate::application::dtos::requests::CopyActionToContainerRequest;
use crate::domain::errors::StepError;

/// Inbound port for copying an action's files into the job's container.
pub trait CopyActionToContainerPort: Send + Sync {
    fn execute(&self, request: CopyActionToContainerRequest) -> Result<String, StepError>;
}
