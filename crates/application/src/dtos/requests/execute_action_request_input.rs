use super::execute_action_execution_input::ExecuteActionExecutionInput;

/// Input containing an action reference, step name, and execution data.
pub struct ExecuteActionRequestInput {
    action_ref: String,
    step: String,
    execution: ExecuteActionExecutionInput,
}

impl ExecuteActionRequestInput {
    /// Creates input from an action reference, step name, and execution data.
    pub fn new(
        action_ref: impl Into<String>,
        step: impl Into<String>,
        execution: ExecuteActionExecutionInput,
    ) -> Self {
        Self {
            action_ref: action_ref.into(),
            step: step.into(),
            execution,
        }
    }

    /// Consumes the input and returns its parts.
    pub fn into_parts(self) -> (String, String, ExecuteActionExecutionInput) {
        (self.action_ref, self.step, self.execution)
    }
}
