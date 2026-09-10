use crate::{
    application::{
        dtos::{ExecuteStepCommand, ExecuteStepRequest, ExecutedStep},
        ports::inbound::execute_step_port::ExecuteStepPort,
    },
    domain::errors::StepError,
};

/// Infrastructure command handler that processes `ExecuteStepCommand`.
pub struct StepCommandHandler {
    executor: Box<dyn ExecuteStepPort>,
}

impl StepCommandHandler {
    pub fn new(executor: Box<dyn ExecuteStepPort>) -> Self {
        Self { executor }
    }

    pub fn handle(&self, cmd: ExecuteStepCommand) -> Result<ExecutedStep, StepError> {
        let (step, env, context, container, repo_path) = cmd.into_parts();
        let req = ExecuteStepRequest::new(&step, &context, container, &repo_path, &env);
        self.executor.execute(req)
    }
}
