use super::super::steps::run_composite_step_port::RunCompositeStepPort;
use crate::application::ports::outbound::run_composite_action_port::RunCompositeActionPort;
use std::collections::HashMap;

use serde_json::Value;

use crate::{
    application::dtos::{
        ExecuteActionResponse, RunCompositeActionRequest, RunCompositeStepRequest,
    },
    domain::{
        errors::StepError,
        expression::{EvalContext, StepInterpolator},
    },
};

/// Service that runs a composite action's steps in order, accumulating their
/// output and stopping at the first one that fails.
pub struct RunCompositeActionService {
    step_runner: Box<dyn RunCompositeStepPort>,
}

impl RunCompositeActionService {
    pub fn new(step_runner: Box<dyn RunCompositeStepPort>) -> Self {
        Self { step_runner }
    }

    /// Returns a copy of `context` whose `inputs` are the action's own.
    fn context_with_inputs(context: &EvalContext, inputs: &HashMap<String, String>) -> EvalContext {
        let input_values = Value::Object(
            inputs
                .iter()
                .map(|(name, value)| (name.clone(), Value::String(value.clone())))
                .collect(),
        );
        context.clone().with_inputs(input_values)
    }

    fn process_step_outcome(
        outcome: Result<crate::application::dtos::ExecResult, StepError>,
        stdout: &mut String,
        stderr: &mut String,
    ) -> Result<Option<ExecuteActionResponse>, StepError> {
        match outcome {
            Ok(result) => {
                stdout.push_str(result.stdout());
                stderr.push_str(result.stderr());
                if result.exit_code() != 0 {
                    Ok(Some(ExecuteActionResponse::new(
                        result.exit_code(),
                        stdout.clone(),
                        stderr.clone(),
                    )))
                } else {
                    Ok(None)
                }
            }
            Err(error) => Err(StepError::new(error.message())
                .with_stdout(format!("{stdout}{}", error.stdout()))
                .with_stderr(format!("{stderr}{}", error.stderr()))),
        }
    }
}

impl RunCompositeActionPort for RunCompositeActionService {
    fn execute(
        &self,
        request: RunCompositeActionRequest<'_>,
    ) -> Result<ExecuteActionResponse, StepError> {
        let context =
            Self::context_with_inputs(request.action_request().context(), request.inputs());
        let mut stdout = String::new();
        let mut stderr = String::new();

        for step in request.steps() {
            let interpolated = StepInterpolator::interpolate(step, &context).map_err(|error| {
                StepError::new(format!("failed to resolve expressions: {error:?}"))
                    .with_stdout(stdout.clone())
                    .with_stderr(stderr.clone())
            })?;

            let outcome = self.step_runner.execute(RunCompositeStepRequest::new(
                &interpolated,
                request.action_dir(),
                request.action_request(),
                &context,
            ));

            if let Some(early_exit) = Self::process_step_outcome(outcome, &mut stdout, &mut stderr)?
            {
                return Ok(early_exit);
            }
        }

        Ok(ExecuteActionResponse::new(0, stdout, stderr))
    }
}
