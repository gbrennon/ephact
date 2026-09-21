use std::sync::Arc;

use crate::{
    application::{
        dtos::{requests::RunCompositeActionRequest, responses::ExecuteActionResponse},
        ports::outbound::container_port::ContainerPort,
    },
    domain::errors::StepError,
};
/// Inbound port for running a composite action's steps.
pub trait RunCompositeActionPort: Send + Sync {
    fn execute(
        &self,
        request: RunCompositeActionRequest<'_>,
        container: Arc<dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError>;
}
