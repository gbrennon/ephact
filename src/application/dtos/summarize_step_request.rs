use std::time::Duration;

use crate::{
    application::dtos::ExecutedStep,
    domain::{errors::StepError, workflow::Step},
};

/// Request DTO for the
/// [`SummarizeStepPort`](crate::application::ports::inbound::summarize_step_port::SummarizeStepPort)
/// inbound port.
pub struct SummarizeStepRequest<'a> {
    /// The step as declared, before its expressions were resolved.
    step: &'a Step,
    /// What executing the step produced.
    outcome: Result<ExecutedStep, StepError>,
    /// How long the step took.
    duration: Duration,
}

impl<'a> SummarizeStepRequest<'a> {
    /// Creates a new request.
    pub fn new(
        step: &'a Step,
        outcome: Result<ExecutedStep, StepError>,
        duration: Duration,
    ) -> Self {
        Self {
            step,
            outcome,
            duration,
        }
    }

    /// The step as declared, before its expressions were resolved.
    pub fn step(&self) -> &'a Step {
        self.step
    }

    /// What executing the step produced.
    pub fn outcome(&self) -> &Result<ExecutedStep, StepError> {
        &self.outcome
    }

    /// Consumes the request and returns what executing the step produced.
    pub fn into_outcome(self) -> Result<ExecutedStep, StepError> {
        self.outcome
    }

    /// How long the step took.
    pub fn duration(&self) -> Duration {
        self.duration
    }
}
