use std::error::Error;

use crate::{dtos::requests::CreateJobContainerRequest, ports::outbound::ContainerPort};

/// Creates the container used to execute a job.
pub trait CreateJobContainerPort: Send + Sync {
    /// Creates a container from the requested image and execution settings.
    ///
    /// Attempts to remove existing containers with the requested current or
    /// legacy name before creation.
    /// When repository writes are allowed, the repository is made available at
    /// the container workspace; otherwise it can be copied separately.
    ///
    /// # Errors
    ///
    /// Returns an error when the runtime cannot create the container.
    fn create(
        &self,
        request: CreateJobContainerRequest,
    ) -> Result<Box<dyn ContainerPort>, Box<dyn Error>>;
}
