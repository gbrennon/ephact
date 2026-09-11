use crate::application::dtos::requests::ExecuteStepRequest;
use crate::application::dtos::responses::ExecutedStepResponse;
use crate::application::ports::inbound::execute_step_port::ExecuteStepPort;
use crate::application::ports::outbound::container_port::ContainerPort;
use crate::domain::errors::StepError;
use crate::domain::messages::commands::ExecuteStepCommand;

/// Infrastructure command handler that processes `ExecuteStepCommand`.
pub struct StepCommandHandler {
    executor: Box<dyn ExecuteStepPort>,
}

impl StepCommandHandler {
    pub fn new(executor: Box<dyn ExecuteStepPort>) -> Self {
        Self { executor }
    }

    pub fn handle<'a>(
        &self,
        cmd: ExecuteStepCommand<'a, dyn ContainerPort>,
    ) -> Result<ExecutedStepResponse, StepError> {
        let (step, env, context, container, repo_path) = cmd.into_parts();
        let req = ExecuteStepRequest::new(&step, &context, container, &repo_path, &env);
        self.executor.execute(req)
    }
}
