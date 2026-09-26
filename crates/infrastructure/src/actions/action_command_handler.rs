use crate::{
    actions::ExecuteActionFactory,
    application::{
        dtos::{
            requests::{
                ExecuteActionExecutionInput, ExecuteActionRequest, ExecuteActionRequestInput,
            },
            responses::ExecuteActionResponse,
        },
        ports::outbound::container_port::ContainerPort,
    },
    domain::{
        errors::StepError,
        messages::commands::ExecuteActionCommand,
        services::{evaluation_context_mapper::EvaluationContextMapper, step_factory::StepFactory},
    },
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
            StepFactory::to_text(&step)?,
            ExecuteActionExecutionInput::new(
                repo_path,
                env,
                EvaluationContextMapper::to_parts(&context),
            ),
        ));
        let executor = (self.executor_factory)(container);
        executor
            .execute(req)
            .map_err(|error| StepError::new(error.to_string()))
    }
}
