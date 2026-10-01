use crate::{domain::errors::StepError, dtos::requests::CopyActionToContainerRequest};

pub trait CopyActionToContainerPort: Send + Sync {
    fn copy(&self, request: CopyActionToContainerRequest) -> Result<String, StepError>;
}
