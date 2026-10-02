use ephact_domain::{entities::Step, value_objects::EvaluationContext};

use crate::errors::EvalError;

/// Evaluates workflow expressions against a runtime context and applies them to steps.
pub trait StepInterpolatorPort: Send + Sync {
    /// Interpolates user-facing step fields while preserving execution controls.
    fn interpolate(&self, step: &Step, context: &EvaluationContext) -> Result<Step, EvalError>;

    /// Evaluates whether a step's `if` condition permits execution.
    fn should_run(&self, step: &Step, context: &EvaluationContext) -> Result<bool, EvalError>;
}
