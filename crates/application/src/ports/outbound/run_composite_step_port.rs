use std::sync::Arc;

use crate::{
    domain::errors::StepError,
    dtos::{requests::RunCompositeStepRequest, responses::ExecResultResponse},
    ports::outbound::ContainerPort,
};

/// Executes one step belonging to a composite action.
pub trait RunCompositeStepPort: Send + Sync {
    /// Publishes nested action steps for action execution and runs shell steps
    /// with the action path available as `GITHUB_ACTION_PATH`.
    ///
    /// # Errors
    ///
    /// Returns [`StepError`] when the nested action or shell step fails.
    fn run(
        &self,
        request: RunCompositeStepRequest<'_>,
        container: Arc<dyn ContainerPort>,
    ) -> Result<ExecResultResponse, StepError>;
}
