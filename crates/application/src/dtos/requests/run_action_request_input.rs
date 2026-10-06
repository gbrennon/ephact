use super::run_action_execution_input::RunActionExecutionInput;

/// Input data for constructing a [`super::run_action_request::RunActionRequest`].
pub struct RunActionRequestInput {
    action_ref: String,
    step: String,
    execution: RunActionExecutionInput,
}

impl RunActionRequestInput {
    /// Creates input from an action reference, step name, and execution data.
    pub fn new(
        action_ref: impl Into<String>,
        step: impl Into<String>,
        execution: RunActionExecutionInput,
    ) -> Self {
        Self {
            action_ref: action_ref.into(),
            step: step.into(),
            execution,
        }
    }

    /// Consumes the input and returns its action reference, step, and execution data.
    pub fn into_parts(self) -> (String, String, RunActionExecutionInput) {
        (self.action_ref, self.step, self.execution)
    }
}
