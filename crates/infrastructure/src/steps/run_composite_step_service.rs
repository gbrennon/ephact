use std::sync::Arc;

use crate::{
    application::{
        dtos::{
            requests::{RunCompositeStepRequest, RunShellStepRequest},
            responses::ExecResultResponse,
        },
        ports::outbound::{
            ActionCommandPublisherPort, container_port::ContainerPort,
            run_composite_step_port::RunCompositeStepPort,
            shell_step_runner_port::ShellStepRunnerPort,
        },
    },
    domain::{errors::StepError, messages::commands::ExecuteActionPayload},
};

/// Runs one step of a composite action: shell steps go straight to the shell
/// runner, while steps referencing another action are published as an
/// [`ExecuteActionPayload`] so the action command handler executes them.
pub struct RunCompositeStepService {
    shell_runner: Box<dyn ShellStepRunnerPort>,
    command_publisher: Box<dyn ActionCommandPublisherPort>,
}

impl RunCompositeStepService {
    pub fn new(
        shell_runner: Box<dyn ShellStepRunnerPort>,
        command_publisher: Box<dyn ActionCommandPublisherPort>,
    ) -> Self {
        Self {
            shell_runner,
            command_publisher,
        }
    }
}

impl RunCompositeStepPort for RunCompositeStepService {
    fn run(
        &self,
        request: RunCompositeStepRequest<'_>,
        container: Arc<dyn ContainerPort>,
    ) -> Result<ExecResultResponse, StepError> {
        let action_request = request.action_request();
        let action_env = request.environment().clone();
        match request.step().uses() {
            Some(nested) => {
                let response = self.command_publisher.publish(
                    ExecuteActionPayload::new(
                        nested.to_string(),
                        request.step().clone(),
                        action_request.repo_path().to_path_buf(),
                        action_env,
                        request.context().clone(),
                    ),
                    container.clone(),
                )?;
                Ok(ExecResultResponse::new(
                    response.exit_code(),
                    response.stdout().to_string(),
                    response.stderr().to_string(),
                ))
            }
            None => {
                let mut action_env = action_env.clone();
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
