use std::{collections::HashMap, sync::Arc};

use crate::{
    application::{
        dtos::{
            requests::{RunCompositeActionRequest, RunCompositeStepRequest},
            responses::ExecuteActionResponse,
        },
        ports::outbound::{
            composite_action_runner_port::CompositeActionRunnerPort, container_port::ContainerPort,
        },
    },
    domain::{
        errors::StepError,
        services::{StepInterpolator, evaluation_context_mapper::EvaluationContextMapper},
        value_objects::{ContextValue, EvaluationContext},
    },
    steps::run_composite_step_port::RunCompositeStepPort,
};

/// Service that runs a composite action's steps in order, accumulating their
/// output and stopping at the first one that fails.
pub struct RunCompositeActionService {
    step_runner: Box<dyn RunCompositeStepPort>,
}

struct ActionOutput {
    stdout: String,
    stderr: String,
}

impl ActionOutput {
    fn new() -> Self {
        Self {
            stdout: String::new(),
            stderr: String::new(),
        }
    }

    fn append(&mut self, result: &crate::application::dtos::responses::ExecResultResponse) {
        self.stdout.push_str(result.stdout());
        self.stderr.push_str(result.stderr());
    }
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
        output: &mut ActionOutput,
    ) -> Result<Option<ExecuteActionResponse>, StepError> {
        for step in request.steps() {
            let interpolated = StepInterpolator::interpolate(step, context).map_err(|error| {
                StepError::new(format!("failed to resolve expressions: {error:?}"))
                    .with_stdout(output.stdout.clone())
                    .with_stderr(output.stderr.clone())
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

            if let Some(early_exit) = Self::process_step_outcome(outcome, output)? {
                return Ok(Some(early_exit));
            }
        }

        Ok(None)
    }

    fn process_step_outcome(
        outcome: Result<crate::application::dtos::responses::ExecResultResponse, StepError>,
        output: &mut ActionOutput,
    ) -> Result<Option<ExecuteActionResponse>, StepError> {
        match outcome {
            Ok(result) => {
                output.append(&result);
                if result.exit_code() != 0 {
                    Ok(Some(ExecuteActionResponse::new(
                        result.exit_code(),
                        output.stdout.clone(),
                        output.stderr.clone(),
                    )))
                } else {
                    Ok(None)
                }
            }
            Err(error) => Err(StepError::new(error.message())
                .with_stdout(format!("{}{}", output.stdout, error.stdout()))
                .with_stderr(format!("{}{}", output.stderr, error.stderr()))),
        }
    }
}

impl CompositeActionRunnerPort for RunCompositeActionService {
    fn run(
        &self,
        request: RunCompositeActionRequest<'_>,
        container: Arc<dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError> {
        let base_context =
            EvaluationContextMapper::from_parts(request.action_request().context().to_vec())
                .map_err(|error| StepError::new(error.to_string()))?;
        let context = Self::context_with_inputs(&base_context, request.inputs());
        let mut output = ActionOutput::new();

        if let Some(early_exit) = self.execute_steps(&request, &context, container, &mut output)? {
            return Ok(early_exit);
        }

        Ok(ExecuteActionResponse::new(0, output.stdout, output.stderr))
    }
}
