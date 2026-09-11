use crate::application::dtos::requests::ExecuteStepRequest;
use crate::application::dtos::requests::RunShellStepRequest;
use crate::application::dtos::responses::ExecuteActionResponse;
use crate::application::dtos::responses::ExecutedStepResponse;
use crate::application::ports::inbound::execute_step_port::ExecuteStepPort;
use crate::application::ports::outbound::command_bus_port::ActionCommandBusPort;
use crate::application::ports::outbound::run_shell_step_port::RunShellStepPort;
use crate::domain::errors::StepError;
use crate::domain::messages::commands::ExecuteActionCommand;
use crate::domain::services::StepInterpolator;

/// Application service coordinating the execution of one step.
///
/// Resolves the step's expressions, then either runs it as a shell script
/// through an outbound port or publishes an [`ExecuteActionCommand`] when the
/// step references an action: the action command handler owns that execution.
pub struct ExecuteStepService {
    shell_runner: Box<dyn RunShellStepPort>,
    command_bus: Box<ActionCommandBusPort>,
}

impl ExecuteStepService {
    pub fn new(
        shell_runner: Box<dyn RunShellStepPort>,
        command_bus: Box<ActionCommandBusPort>,
    ) -> Self {
        Self {
            shell_runner,
            command_bus,
        }
    }
}

impl ExecuteStepPort for ExecuteStepService {
    fn execute(&self, request: ExecuteStepRequest<'_>) -> Result<ExecutedStepResponse, StepError> {
        let interpolated = StepInterpolator::interpolate(request.step(), request.context())
            .map_err(|error| StepError::new(format!("failed to resolve expressions: {error:?}")))?;

        let response = match interpolated.uses() {
            Some(action_ref) => self.command_bus.dispatch(ExecuteActionCommand::new(
                action_ref.to_string(),
                interpolated.clone(),
                request.repo_path().to_path_buf(),
                request.env().clone(),
                request.context().clone(),
                request.container(),
            ))?,
            None => {
                let result = self.shell_runner.execute(RunShellStepRequest::new(
                    &interpolated,
                    request.container(),
                    request.env(),
                ))?;
                ExecuteActionResponse::new(result.exit_code(), result.stdout(), result.stderr())
            }
        };

        Ok(ExecutedStepResponse::new(interpolated, response))
    }
}
