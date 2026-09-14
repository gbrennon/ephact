use crate::application::dtos::requests::CopyRepositoryToContainerRequest;
use crate::application::ports::outbound::container_port::ContainerPort;
use std::error::Error;

pub trait CopyRepositoryToContainerPort: Send + Sync {
    fn execute(
        &self,
        request: CopyRepositoryToContainerRequest<'_>,
        container: &dyn ContainerPort,
    ) -> Result<(), Box<dyn Error>>;
}
