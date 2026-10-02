use std::{
    collections::{BTreeMap, HashMap},
    path::Path,
    sync::Arc,
};

use crate::{
    application::{
        dtos::{
            requests::{
                CopyActionToContainerRequest, RunCompositeActionRequest, RunCompositeStepRequest,
            },
            responses::ExecuteActionResponse,
        },
        ports::outbound::{
            composite_action_runner_port::CompositeActionRunnerPort, container_port::ContainerPort,
            copy_action_to_container_port::CopyActionToContainerPort,
            run_composite_step_port::RunCompositeStepPort,
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
    pub fn new(
        step_runner: Box<dyn RunCompositeStepPort>,
        action_copier: Box<dyn CopyActionToContainerPort>,
    ) -> Self {
        Self {
            step_runner,
            action_copier,
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
        container: &dyn ContainerPort,
        step: &Step,
        environment: &mut HashMap<String, String>,
        context: &mut EvaluationContext,
    ) {
        if let Some(path_file) = environment.get("GITHUB_PATH").cloned()
            && let Ok(result) = container.exec(&["cat".into(), path_file], None, &HashMap::new())
        {
            let current_path = environment.get("PATH").cloned().unwrap_or_default();
            let additions: Vec<_> = result
                .stdout()
                .lines()
                .map(str::trim)
                .filter(|path| {
                    !path.is_empty() && !current_path.split(':').any(|entry| entry == *path)
                })
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
        }

        if let Some(env_file) = environment.get("GITHUB_ENV").cloned()
            && let Ok(result) = container.exec(&["cat".into(), env_file], None, &HashMap::new())
        {
            for line in result.stdout().lines().map(str::trim) {
                if let Some((key, value)) = line.split_once('=') {
                    environment.insert(key.to_owned(), value.to_owned());
                }
            }
        }
        let Some(step_id) = step.id() else {
            return;
        };
        let Some(output_file) = environment.get("GITHUB_OUTPUT").cloned() else {
            return;
        };
        let Ok(result) = container.exec(&["cat".into(), output_file], None, &HashMap::new()) else {
            return;
        };
        let outputs: BTreeMap<_, _> = result
            .stdout()
            .lines()
            .filter_map(|line| {
                line.split_once('=')
                    .map(|(key, value)| (key.to_owned(), ContextValue::text(value)))
            })
            .collect();
        if outputs.is_empty() {
            return;
        }
        let mut steps = match context.get("steps") {
            Some(ContextValue::Mapping(values)) => values.clone(),
            _ => BTreeMap::new(),
        };
        let step_value =
            ContextValue::mapping([("outputs".to_owned(), ContextValue::Mapping(outputs))]);
        steps.insert(step_id.to_owned(), step_value);
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
        for step in request.steps() {
            let should_run = StepInterpolator::should_run(step, context).map_err(|error| {
                StepError::new(format!("failed to evaluate step condition: {error:?}"))
                    .with_stdout(output.stdout.clone())
                    .with_stderr(output.stderr.clone())
            })?;
            if !should_run {
                continue;
            }

            let interpolated = StepInterpolator::interpolate(step, context).map_err(|error| {
                StepError::new(format!("failed to resolve expressions: {error:?}"))
                    .with_stdout(output.stdout.clone())
                    .with_stderr(output.stderr.clone())
            })?;
            let outcome = self.step_runner.run(
                RunCompositeStepRequest::new(
                    &interpolated,
                    action_dir,
                    request.action_request(),
                    context,
                )
                .with_environment(&environment),
                container.clone(),
            );

            if let Some(early_exit) = Self::process_step_outcome(outcome, output)? {
                return Ok(Some(early_exit));
            }
            Self::merge_runner_exports(container.as_ref(), step, &mut environment, context);
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
