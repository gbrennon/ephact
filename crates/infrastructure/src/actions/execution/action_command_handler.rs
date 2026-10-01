use crate::{
    actions::ExecuteActionFactory,
    application::{
        dtos::{
            requests::{
                ExecuteActionExecutionInput, ExecuteActionRequest, ExecuteActionRequestInput,
            },
            responses::ExecuteActionResponse,
        },
        ports::outbound::{StepTextCodecPort, container_port::ContainerPort},
    },
    domain::{errors::StepError, messages::commands::ExecuteActionCommand},
    steps::JsonStepTextCodec,
};

/// Infrastructure command handler that processes `ExecuteActionCommand`.
pub struct ActionCommandHandler {
    executor_factory: ExecuteActionFactory,
}

impl ActionCommandHandler {
    pub fn new(executor_factory: ExecuteActionFactory) -> Self {
        Self { executor_factory }
    }

    pub fn handle(
        &self,
        cmd: ExecuteActionCommand<dyn ContainerPort>,
    ) -> Result<ExecuteActionResponse, StepError> {
        let (action_ref, step, repo_path, env, context, container) = cmd.into_parts();
        let req = ExecuteActionRequest::new(ExecuteActionRequestInput::new(
            action_ref,
            JsonStepTextCodec.encode(&step)?,
            ExecuteActionExecutionInput::new(repo_path, env, context),
        ));
        let executor = (self.executor_factory)(container);
        executor.execute(req).map_err(|error| {
            StepError::new(error.message())
                .with_stdout(error.stdout().to_string())
                .with_stderr(error.stderr().to_string())
        })
    }
}
