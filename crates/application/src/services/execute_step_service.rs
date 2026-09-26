use std::sync::Arc;

use crate::{
    domain::{
        errors::StepError,
        messages::commands::ExecuteActionCommand,
        services::{StepInterpolator, evaluation_context_mapper::EvaluationContextMapper},
    },
    dtos::{
        requests::{ExecuteStepRequest, RunShellStepRequest},
        responses::{ExecuteActionResponse, ExecutedStepResponse},
    },
    errors::ExecuteStepError,
    ports::{
        inbound::execute_step_port::ExecuteStepPort,
        outbound::{
            ContainerPort, action_command_bus_port::ActionCommandBusPort,
            shell_step_runner_port::ShellStepRunnerPort, step_text_codec_port::StepTextCodecPort,
        },
    },
};

pub struct ExecuteStepService {
    container: Arc<dyn ContainerPort>,
    shell_runner: Arc<dyn ShellStepRunnerPort>,
    command_bus: Arc<dyn ActionCommandBusPort>,
    step_codec: Arc<dyn StepTextCodecPort>,
}

impl ExecuteStepService {
    pub fn new(
        container: Arc<dyn ContainerPort>,
        shell_runner: Arc<dyn ShellStepRunnerPort>,
        command_bus: Arc<dyn ActionCommandBusPort>,
        step_codec: Arc<dyn StepTextCodecPort>,
    ) -> Self {
        Self {
            container,
            shell_runner,
            command_bus,
            step_codec,
        }
    }

    fn execute_interpolated_step(
        &self,
        step: &crate::domain::entities::Step,
        request: &ExecuteStepRequest,
        context: &crate::domain::value_objects::EvaluationContext,
    ) -> Result<ExecuteActionResponse, StepError> {
        if let Some(action_ref) = step.uses() {
            return self.command_bus.dispatch(
                ExecuteActionCommand::new(
                    action_ref.to_string(),
                    step.clone(),
                    request.repo_path().to_path_buf(),
                    request.env().clone(),
                    self.container.clone(),
                )
                .with_context(context.clone()),
            );
        }

        let result = self.shell_runner.run(RunShellStepRequest::new(
            step,
            self.container.as_ref(),
            request.env(),
        ))?;
        Ok(ExecuteActionResponse::new(
            result.exit_code(),
            result.stdout(),
            result.stderr(),
        ))
    }
}

impl ExecuteStepPort for ExecuteStepService {
    fn execute(
        &self,
        request: ExecuteStepRequest,
    ) -> Result<ExecutedStepResponse, ExecuteStepError> {
        let step = self
            .step_codec
            .decode(request.step())
            .map_err(ExecuteStepError::Step)?;
        let context = EvaluationContextMapper::from_parts(request.context().to_vec())
            .map_err(|error| ExecuteStepError::Step(StepError::new(error.to_string())))?;
        let interpolated = StepInterpolator::interpolate(&step, &context).map_err(|error| {
            ExecuteStepError::Step(StepError::new(format!(
                "failed to resolve expressions: {error:?}"
            )))
        })?;
        let response = self
            .execute_interpolated_step(&interpolated, &request, &context)
            .map_err(ExecuteStepError::Step)?;
        Ok(ExecutedStepResponse::new(interpolated, response))
    }
}
