use crate::{
    dtos::requests::CopyRepositoryToContainerRequest, errors::CopyRepositoryToContainerError,
    ports::outbound::ContainerPort,
};

pub trait CopyRepositoryToContainerPort: Send + Sync {
    fn copy(
        &self,
        request: CopyRepositoryToContainerRequest,
        container: &dyn ContainerPort,
    ) -> Result<(), CopyRepositoryToContainerError>;
}
