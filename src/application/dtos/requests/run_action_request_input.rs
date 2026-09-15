use super::run_action_execution_input::RunActionExecutionInput;

pub struct RunActionRequestInput {
    action_ref: String,
    step: String,
    execution: RunActionExecutionInput,
}

impl RunActionRequestInput {
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

    pub fn into_parts(self) -> (String, String, RunActionExecutionInput) {
        (self.action_ref, self.step, self.execution)
    }
}
