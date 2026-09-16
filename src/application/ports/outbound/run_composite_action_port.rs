use std::sync::Arc;

use crate::application::dtos::requests::RunCompositeActionRequest;
use crate::application::dtos::responses::ExecuteActionResponse;
use crate::application::ports::outbound::container_port::ContainerPort;
use crate::domain::errors::StepError;
/// Inbound port for running a composite action's steps.
pub trait RunCompositeActionPort: Send + Sync {
    fn execute(
        &self,
        request: RunCompositeActionRequest<'_>,
        container: Arc<dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError>;
}
