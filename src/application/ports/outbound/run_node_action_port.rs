use std::sync::Arc;

use crate::{
    application::{
        dtos::{requests::RunNodeActionRequest, responses::RunNodeActionResponse},
        ports::outbound::container_port::ContainerPort,
    },
    domain::errors::StepError,
};

/// Inbound port for running a JavaScript action inside the job's container.
pub trait RunNodeActionPort: Send + Sync {
    fn execute(
        &self,
        request: RunNodeActionRequest,
        container: Arc<dyn ContainerPort>,
    ) -> Result<RunNodeActionResponse, StepError>;
}
