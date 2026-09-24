use std::sync::Arc;

use crate::{
    application::{
        dtos::{
            requests::{RunCompositeStepRequest, RunShellStepRequest},
            responses::ExecResultResponse,
        },
        ports::outbound::{
            action_command_bus_port::ActionCommandBusPort, container_port::ContainerPort,
            shell_step_runner_port::ShellStepRunnerPort,
        },
    },
    domain::{errors::StepError, messages::commands::ExecuteActionCommand},
    infrastructure::steps::run_composite_step_port::RunCompositeStepPort,
};

/// Runs one step of a composite action: shell steps go straight to the shell
/// runner, while steps referencing another action are published as an
/// [`ExecuteActionCommand`] so the action command handler executes them.
pub struct RunCompositeStepService {
    shell_runner: Box<dyn ShellStepRunnerPort>,
    command_bus: Box<dyn ActionCommandBusPort>,
}

impl RunCompositeStepService {
    pub fn new(
        shell_runner: Box<dyn ShellStepRunnerPort>,
        command_bus: Box<dyn ActionCommandBusPort>,
    ) -> Self {
        Self {
            shell_runner,
            command_bus,
        }
    }
}

impl RunCompositeStepPort for RunCompositeStepService {
    fn execute(
        &self,
        request: RunCompositeStepRequest<'_>,
        container: Arc<dyn ContainerPort>,
    ) -> Result<ExecResultResponse, StepError> {
        let action_request = request.action_request();
        match request.step().uses() {
            Some(nested) => self
                .command_bus
                .dispatch(
                    ExecuteActionCommand::new(
                        nested.to_string(),
                        request.step().clone(),
                        action_request.repo_path().to_path_buf(),
                        action_request.env().clone(),
                        container.clone(),
                    )
                    .with_context(request.context().clone()),
                )
                .map(|response| {
                    ExecResultResponse::new(
                        response.exit_code(),
                        response.stdout().to_string(),
                        response.stderr().to_string(),
                    )
                }),
            None => {
                let mut action_env = action_request.env().clone();
                action_env.insert(
                    "GITHUB_ACTION_PATH".into(),
                    request.action_dir().display().to_string(),
                );
                self.shell_runner.run(RunShellStepRequest::new(
                    request.step(),
                    container.as_ref(),
                    &action_env,
                ))
            }
        }
    }
}
