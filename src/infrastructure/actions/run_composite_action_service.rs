use std::{collections::HashMap, sync::Arc};

use super::super::steps::run_composite_step_port::RunCompositeStepPort;
use crate::{
    application::{
        dtos::{
            requests::{RunCompositeActionRequest, RunCompositeStepRequest},
            responses::ExecuteActionResponse,
        },
        ports::outbound::{
            container_port::ContainerPort, run_composite_action_port::RunCompositeActionPort,
        },
    },
    domain::{
        errors::StepError,
        services::{StepInterpolator, evaluation_context_mapper::EvaluationContextMapper},
        value_objects::{ContextValue, EvaluationContext},
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
    fn context_with_inputs(
        context: &EvaluationContext,
        inputs: &HashMap<String, String>,
    ) -> EvaluationContext {
        let input_values = ContextValue::mapping(
            inputs
                .iter()
                .map(|(name, value)| (name.clone(), ContextValue::text(value.clone()))),
        );
        context.clone().with_inputs(input_values)
    }

    fn execute_steps(
        &self,
        request: &RunCompositeActionRequest<'_>,
        context: &EvaluationContext,
        container: Arc<dyn ContainerPort>,
        stdout: &mut String,
        stderr: &mut String,
    ) -> Result<Option<ExecuteActionResponse>, StepError> {
        for step in request.steps() {
            let interpolated = StepInterpolator::interpolate(step, context).map_err(|error| {
                StepError::new(format!("failed to resolve expressions: {error:?}"))
                    .with_stdout(stdout.clone())
                    .with_stderr(stderr.clone())
            })?;

            let outcome = self.step_runner.execute(
                RunCompositeStepRequest::new(
                    &interpolated,
                    request.action_dir(),
                    request.action_request(),
                    context,
                ),
                container.clone(),
            );

            if let Some(early_exit) = Self::process_step_outcome(outcome, stdout, stderr)? {
                return Ok(Some(early_exit));
            }
        }

        Ok(None)
    }

    fn process_step_outcome(
        outcome: Result<crate::application::dtos::responses::ExecResultResponse, StepError>,
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
        container: Arc<dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError> {
        let base_context =
            EvaluationContextMapper::from_parts(request.action_request().context().to_vec())
                .map_err(|error| StepError::new(error.to_string()))?;
        let context = Self::context_with_inputs(&base_context, request.inputs());
        let mut stdout = String::new();
        let mut stderr = String::new();

        if let Some(early_exit) =
            self.execute_steps(&request, &context, container, &mut stdout, &mut stderr)?
        {
            return Ok(early_exit);
        }

        Ok(ExecuteActionResponse::new(0, stdout, stderr))
    }
}
