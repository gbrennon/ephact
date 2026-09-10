use crate::application::dtos::requests::MergeRunExecutionsRequest;
use crate::application::dtos::responses::WorkflowExecutionResponse;

/// Inbound port for reducing a run's workflow executions to the one execution
/// the run reports.
pub trait MergeRunExecutionsPort: Send + Sync {
    /// Merges the executions the run produced.
    fn execute(
        &self,
        request: MergeRunExecutionsRequest,
    ) -> Result<WorkflowExecutionResponse, Box<dyn std::error::Error>>;
}
