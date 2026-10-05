use crate::{domain::errors::StepError, dtos::requests::CopyActionToContainerRequest};

/// Copies an action's files into a container for execution.
pub trait CopyActionToContainerPort: Send + Sync {
    /// Collects the action directory, copies its files to the supplied
    /// container, and returns the container directory used for the copy.
    ///
    /// # Errors
    ///
    /// Returns [`StepError`] when files cannot be collected or copied.
    fn copy(&self, request: CopyActionToContainerRequest) -> Result<String, StepError>;
}
