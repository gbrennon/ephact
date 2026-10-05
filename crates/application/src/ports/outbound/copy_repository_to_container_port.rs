use crate::{
    dtos::requests::CopyRepositoryToContainerRequest, errors::CopyRepositoryToContainerError,
    ports::outbound::ContainerPort,
};

/// Copies repository files into a container.
pub trait CopyRepositoryToContainerPort: Send + Sync {
    /// Recursively copies the repository to the requested container path.
    /// Implementations omit generated and development-only directories from
    /// the transfer.
    ///
    /// # Errors
    ///
    /// Returns [`CopyRepositoryToContainerError`] when repository files cannot
    /// be read or the container transfer fails.
    fn copy(
        &self,
        request: CopyRepositoryToContainerRequest,
        container: &dyn ContainerPort,
    ) -> Result<(), CopyRepositoryToContainerError>;
}
