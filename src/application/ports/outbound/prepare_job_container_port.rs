use crate::application::dtos::requests::PrepareJobContainerRequest;
use crate::application::dtos::responses::PreparedJobContainerResponse;
use crate::application::errors::PrepareJobContainerError;

pub trait PrepareJobContainerPort: Send + Sync {
    fn execute(
        &self,
        request: PrepareJobContainerRequest,
    ) -> Result<PreparedJobContainerResponse, PrepareJobContainerError>;
}
