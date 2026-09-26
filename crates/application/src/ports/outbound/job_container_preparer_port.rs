use crate::{
    dtos::{requests::PrepareJobContainerRequest, responses::PreparedJobContainerResponse},
    errors::PrepareJobContainerError,
};
/// Prepares the container required to run a job.
pub trait JobContainerPreparerPort: Send + Sync {
    fn prepare(
        &self,
        request: PrepareJobContainerRequest,
    ) -> Result<PreparedJobContainerResponse, PrepareJobContainerError>;
}
