use crate::application::dtos::requests::{
    ExecuteActionExecutionInput, ExecuteActionRequest, ExecuteActionRequestInput,
};
use crate::application::dtos::responses::ExecuteActionResponse;
use crate::application::ports::outbound::container_port::ContainerPort;
use crate::domain::errors::StepError;
use crate::domain::messages::commands::ExecuteActionCommand;
use crate::domain::services::evaluation_context_mapper::EvaluationContextMapper;
use crate::domain::services::step_factory::StepFactory;
use crate::infrastructure::actions::ExecuteActionFactory;

/// Infrastructure command handler that processes `ExecuteActionCommand`.
pub struct ActionCommandHandler {
    executor_factory: ExecuteActionFactory,
}

impl ActionCommandHandler {
    pub fn new(executor_factory: ExecuteActionFactory) -> Self {
        Self { executor_factory }
    }

    pub fn handle<'a>(
        &self,
        cmd: ExecuteActionCommand<'a, dyn ContainerPort>,
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
