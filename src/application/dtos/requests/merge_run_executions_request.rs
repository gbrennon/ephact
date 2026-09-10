use crate::application::dtos::responses::WorkflowExecutionResponse;

/// Request DTO for the
/// [`MergeRunExecutionsPort`](crate::application::ports::inbound::merge_run_executions_port::MergeRunExecutionsPort)
/// inbound port.
pub struct MergeRunExecutionsRequest {
    /// Executions to merge, in the order they ran.
    executions: Vec<WorkflowExecutionResponse>,
    /// Whether the run covered every workflow of the repository.
    all_workflows: bool,
}

impl MergeRunExecutionsRequest {
    /// Creates a new request.
    pub fn new(executions: Vec<WorkflowExecutionResponse>, all_workflows: bool) -> Self {
        Self {
            executions,
            all_workflows,
        }
    }

    /// Executions to merge, in the order they ran.
    pub fn executions(&self) -> &[WorkflowExecutionResponse] {
        &self.executions
    }

    /// Consumes the request and returns the executions.
    pub fn into_executions(self) -> Vec<WorkflowExecutionResponse> {
        self.executions
    }

    /// Whether the run covered every workflow of the repository.
    pub fn all_workflows(&self) -> bool {
        self.all_workflows
    }
}
