use std::sync::{Arc, Mutex};

use ephact::{
    application::{errors::EvalError, ports::outbound::StepInterpolatorPort},
    domain::{entities::Step, value_objects::EvaluationContext},
};

#[derive(Clone)]
pub struct FakeStepInterpolatorPort {
    interpolated_step: Option<Step>,
    error: Option<EvalError>,
    calls: Arc<Mutex<usize>>,
}
impl FakeStepInterpolatorPort {
    pub fn identity() -> Self {
        Self {
            interpolated_step: None,
            error: None,
            calls: Arc::new(Mutex::new(0)),
        }
    }

    pub fn failing(error: EvalError) -> Self {
        Self {
            interpolated_step: None,
            error: Some(error),
            calls: Arc::new(Mutex::new(0)),
        }
    }

    pub fn returning(step: Step) -> Self {
        Self {
            interpolated_step: Some(step),
            error: None,
            calls: Arc::new(Mutex::new(0)),
        }
    }

    pub fn calls(&self) -> usize {
        *self
            .calls
            .lock()
            .expect("step interpolator fake lock poisoned")
    }
}

impl StepInterpolatorPort for FakeStepInterpolatorPort {
    fn interpolate(&self, step: &Step, _context: &EvaluationContext) -> Result<Step, EvalError> {
        *self
            .calls
            .lock()
            .expect("step interpolator fake lock poisoned") += 1;
        if let Some(error) = &self.error {
            return Err(error.clone());
        }
        Ok(self
            .interpolated_step
            .clone()
            .unwrap_or_else(|| step.clone()))
    }

    fn should_run(&self, _step: &Step, _context: &EvaluationContext) -> Result<bool, EvalError> {
        Ok(true)
    }
}
