use crate::application::{
    dtos::{requests::PrepareJobContainerRequest, responses::PreparedJobContainerResponse},
    errors::PrepareJobContainerError,
};

pub trait PrepareJobContainerPort: Send + Sync {
    fn execute(
        &self,
        request: PrepareJobContainerRequest,
    ) -> Result<PreparedJobContainerResponse, PrepareJobContainerError>;
}
