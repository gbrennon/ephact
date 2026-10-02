use std::{
    collections::{BTreeMap, HashMap},
    path::Path,
    sync::Arc,
};

use super::composite_step_execution::CompositeStepExecution;
use crate::{
    application::{
        dtos::{
            requests::{
                CopyActionToContainerRequest, ReadStepExportsRequest, RunCompositeActionRequest,
                RunCompositeStepRequest,
            },
            responses::ExecuteActionResponse,
        },
        ports::outbound::{
            composite_action_runner_port::CompositeActionRunnerPort, container_port::ContainerPort,
            copy_action_to_container_port::CopyActionToContainerPort,
            run_composite_step_port::RunCompositeStepPort,
            step_exports_reader_port::StepExportsReaderPort,
        },
        services::StepInterpolator,
    },
    domain::{
        entities::Step,
        errors::StepError,
        value_objects::{ContextValue, EvaluationContext},
    },
};
/// Service that runs a composite action's steps in order, accumulating their
/// output and stopping at the first one that fails.
pub struct RunCompositeActionService {
    step_runner: Box<dyn RunCompositeStepPort>,
    action_copier: Box<dyn CopyActionToContainerPort>,
    step_exports_reader: Box<dyn StepExportsReaderPort>,
}
pub(super) struct ActionOutput {
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
    pub fn new(
        step_runner: Box<dyn RunCompositeStepPort>,
        action_copier: Box<dyn CopyActionToContainerPort>,
        step_exports_reader: Box<dyn StepExportsReaderPort>,
    ) -> Self {
        Self {
            step_runner,
            action_copier,
            step_exports_reader,
        }
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
        context.clone().with_root("inputs", input_values)
    }

    fn context_with_action_path(
        context: &EvaluationContext,
        action_dir: &Path,
    ) -> EvaluationContext {
        let mut github = match context.get("github") {
            Some(ContextValue::Mapping(values)) => values.clone(),
            _ => BTreeMap::new(),
        };
        github.insert(
            "action_path".to_owned(),
            ContextValue::text(action_dir.display().to_string()),
        );
        context
            .clone()
            .with_root("github", ContextValue::Mapping(github))
    }

    fn merge_runner_exports(
        exports_reader: &dyn StepExportsReaderPort,
        container: &dyn ContainerPort,
        step: &Step,
        environment: &mut HashMap<String, String>,
        context: &mut EvaluationContext,
    ) {
        let exports = exports_reader.read(ReadStepExportsRequest::new(), container);
        let current_path = environment.get("PATH").cloned().unwrap_or_default();
        let additions: Vec<_> = exports
            .path_additions()
            .iter()
            .filter(|path| !current_path.split(':').any(|entry| entry == path.as_str()))
            .cloned()
            .collect();
        if !additions.is_empty() {
            let prefix = additions.join(":");
            let path = if current_path.is_empty() {
                prefix
            } else {
                format!("{prefix}:{current_path}")
            };
            environment.insert("PATH".into(), path);
        }
        environment.extend(exports.env().clone());

        let Some(step_id) = step.id() else {
            return;
        };
        if exports.outputs().is_empty() {
            return;
        }
        let mut steps = match context.get("steps") {
            Some(ContextValue::Mapping(values)) => values.clone(),
            _ => BTreeMap::new(),
        };
        let outputs = ContextValue::mapping(
            exports
                .outputs()
                .iter()
                .map(|(name, value)| (name.clone(), ContextValue::text(value.clone()))),
        );
        steps.insert(
            step_id.to_owned(),
            ContextValue::mapping([("outputs".to_owned(), outputs)]),
        );
        *context = context
            .clone()
            .with_root("steps", ContextValue::Mapping(steps));
    }

    fn execute_steps(
        &self,
        request: &RunCompositeActionRequest<'_>,
        action_dir: &Path,
        context: &mut EvaluationContext,
        container: Arc<dyn ContainerPort>,
        output: &mut ActionOutput,
    ) -> Result<Option<ExecuteActionResponse>, StepError> {
        let mut environment = request.action_request().env().clone();
        environment.insert(
            "GITHUB_ACTION_PATH".to_owned(),
            action_dir.display().to_string(),
        );
        let mut execution =
            CompositeStepExecution::new(action_dir, context, output, &mut environment);
        for step in request.steps() {
            if let Some(early_exit) =
                self.execute_step(request, &mut execution, container.clone(), step)?
            {
                return Ok(Some(early_exit));
            }
        }
        Ok(None)
    }

    fn interpolated_step(
        step: &Step,
        context: &EvaluationContext,
        output: &ActionOutput,
    ) -> Result<Option<Step>, StepError> {
        let should_run = StepInterpolator::should_run(step, context).map_err(|error| {
            StepError::new(format!("failed to evaluate step condition: {error:?}"))
                .with_stdout(output.stdout.clone())
                .with_stderr(output.stderr.clone())
        })?;
        if !should_run {
            return Ok(None);
        }
        StepInterpolator::interpolate(step, context)
            .map(Some)
            .map_err(|error| {
                StepError::new(format!("failed to resolve expressions: {error:?}"))
                    .with_stdout(output.stdout.clone())
                    .with_stderr(output.stderr.clone())
            })
    }

    fn execute_step(
        &self,
        request: &RunCompositeActionRequest<'_>,
        execution: &mut CompositeStepExecution<'_>,
        container: Arc<dyn ContainerPort>,
        step: &Step,
    ) -> Result<Option<ExecuteActionResponse>, StepError> {
        let action_dir = execution.action_dir().to_path_buf();
        let environment = execution.environment().clone();
        let Some(interpolated) =
            Self::interpolated_step(step, execution.context(), execution.output())?
        else {
            return Ok(None);
        };
        let outcome = self.step_runner.run(
            RunCompositeStepRequest::new(
                &interpolated,
                &action_dir,
                request.action_request(),
                execution.context(),
            )
            .with_environment(&environment),
            container.clone(),
        );

        if let Some(early_exit) = Self::process_step_outcome(outcome, execution.output_mut())? {
            return Ok(Some(early_exit));
        }
        let (environment, context) = execution.environment_and_context_mut();
        Self::merge_runner_exports(
            self.step_exports_reader.as_ref(),
            container.as_ref(),
            step,
            environment,
            context,
        );
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
        let container_action_dir = self.action_copier.copy(CopyActionToContainerRequest::new(
            request.action_dir().to_path_buf(),
            container.clone(),
        ))?;
        let base_context = request.action_request().context().clone();
        let context = Self::context_with_inputs(&base_context, request.inputs());
        let mut context =
            Self::context_with_action_path(&context, Path::new(&container_action_dir));
        let mut output = ActionOutput::new();

        if let Some(early_exit) = self.execute_steps(
            &request,
            Path::new(&container_action_dir),
            &mut context,
            container,
            &mut output,
        )? {
            return Ok(early_exit);
        }

        Ok(ExecuteActionResponse::new(0, output.stdout, output.stderr))
    }
}
