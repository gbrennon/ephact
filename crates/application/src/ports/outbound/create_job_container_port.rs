use std::error::Error;

use crate::{dtos::requests::CreateJobContainerRequest, ports::outbound::ContainerPort};

pub trait CreateJobContainerPort: Send + Sync {
    fn create(
        &self,
        request: CreateJobContainerRequest,
    ) -> Result<Box<dyn ContainerPort>, Box<dyn Error>>;
}
