use crate::{application::dtos::ExecuteActionResponse, domain::entities::Step};

/// Outcome of executing one step, with the step its expressions resolved to.
#[derive(Debug)]
pub struct ExecutedStep {
    step: Step,
    response: ExecuteActionResponse,
}

impl ExecutedStep {
    pub fn new(step: Step, response: ExecuteActionResponse) -> Self {
        Self { step, response }
    }

    pub fn step(&self) -> &Step {
        &self.step
    }

    pub fn response(&self) -> &ExecuteActionResponse {
        &self.response
    }

    pub fn into_parts(self) -> (Step, ExecuteActionResponse) {
        (self.step, self.response)
    }
}
