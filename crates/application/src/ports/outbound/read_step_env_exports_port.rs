use std::collections::HashMap;

use crate::{dtos::requests::ReadStepEnvExportsRequest, ports::outbound::ContainerPort};

pub trait ReadStepEnvExportsPort: Send + Sync {
    fn read(
        &self,
        request: ReadStepEnvExportsRequest,
        container: &dyn ContainerPort,
    ) -> HashMap<String, String>;
}
