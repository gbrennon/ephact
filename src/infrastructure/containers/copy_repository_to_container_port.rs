use crate::application::dtos::requests::CopyRepositoryToContainerRequest;
use crate::application::errors::CopyRepositoryToContainerError;
use crate::application::ports::outbound::container_port::ContainerPort;

pub trait CopyRepositoryToContainerPort: Send + Sync {
    fn execute(
        &self,
        request: CopyRepositoryToContainerRequest,
        container: &dyn ContainerPort,
    ) -> Result<(), CopyRepositoryToContainerError>;
}
