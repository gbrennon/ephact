use super::execute_action_execution_input::ExecuteActionExecutionInput;

pub struct ExecuteActionRequestInput {
    action_ref: String,
    step: String,
    execution: ExecuteActionExecutionInput,
}

impl ExecuteActionRequestInput {
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

    pub fn into_parts(self) -> (String, String, ExecuteActionExecutionInput) {
        (self.action_ref, self.step, self.execution)
    }
}
