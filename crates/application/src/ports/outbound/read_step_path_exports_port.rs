use crate::{dtos::requests::ReadStepPathExportsRequest, ports::outbound::ContainerPort};

pub trait ReadStepPathExportsPort: Send + Sync {
    fn read(
        &self,
        request: ReadStepPathExportsRequest,
        container: &dyn ContainerPort,
    ) -> Vec<String>;
}
