use crate::application::{
    dtos::requests::CopyRepositoryToContainerRequest, errors::CopyRepositoryToContainerError,
    ports::outbound::container_port::ContainerPort,
};

pub trait CopyRepositoryToContainerPort: Send + Sync {
    fn execute(
        &self,
        request: CopyRepositoryToContainerRequest,
        container: &dyn ContainerPort,
    ) -> Result<(), CopyRepositoryToContainerError>;
}
