use std::sync::Arc;

use crate::application::dtos::requests::ExecuteStepRequest;
use crate::application::dtos::requests::RunShellStepRequest;
use crate::application::dtos::responses::ExecuteActionResponse;
use crate::application::dtos::responses::ExecutedStepResponse;
use crate::application::errors::ExecuteStepError;
use crate::application::ports::inbound::execute_step_port::ExecuteStepPort;
use crate::application::ports::outbound::ContainerPort;
use crate::application::ports::outbound::action_command_bus_port::ActionCommandBusPort;
use crate::application::ports::outbound::run_shell_step_port::RunShellStepPort;
use crate::domain::errors::StepError;
use crate::domain::messages::commands::ExecuteActionCommand;
use crate::domain::services::StepInterpolator;
use crate::domain::services::evaluation_context_mapper::EvaluationContextMapper;
use crate::domain::services::step_factory::StepFactory;

pub struct ExecuteStepService<'container> {
    container: &'container dyn ContainerPort,
    shell_runner: Arc<dyn RunShellStepPort>,
    command_bus: Arc<dyn ActionCommandBusPort>,
}

impl<'container> ExecuteStepService<'container> {
    pub fn new(
        container: &'container dyn ContainerPort,
        shell_runner: Arc<dyn RunShellStepPort>,
        command_bus: Arc<dyn ActionCommandBusPort>,
    ) -> Self {
        Self {
            container,
            shell_runner,
            command_bus,
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
                    self.container,
                )
                .with_context(context.clone()),
            );
        }

        let result = self.shell_runner.execute(RunShellStepRequest::new(
            step,
            self.container,
            request.env(),
        ))?;
        Ok(ExecuteActionResponse::new(
            result.exit_code(),
            result.stdout(),
            result.stderr(),
        ))
    }
}

impl ExecuteStepPort for ExecuteStepService<'_> {
    fn execute(
        &self,
        request: ExecuteStepRequest,
    ) -> Result<ExecutedStepResponse, ExecuteStepError> {
        let step = StepFactory::from_text(request.step()).map_err(ExecuteStepError::Step)?;
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
