use crate::{
    actions::ActionCommandHandler,
    application::ports::outbound::{Command, CommandError, CommandHandlerPort, CommandResponse},
    jobs::JobCommandHandler,
    steps::StepCommandHandler,
    workflows::WorkflowCommandHandler,
};

pub struct CommandHandlerAdapter {
    workflow_handler: WorkflowCommandHandler,
    job_handler: JobCommandHandler,
    step_handler: StepCommandHandler,
    action_handler: ActionCommandHandler,
}

impl CommandHandlerAdapter {
    pub fn new(
        workflow_handler: WorkflowCommandHandler,
        job_handler: JobCommandHandler,
        step_handler: StepCommandHandler,
        action_handler: ActionCommandHandler,
    ) -> Self {
        Self {
            workflow_handler,
            job_handler,
            step_handler,
            action_handler,
        }
    }
}

impl CommandHandlerPort for CommandHandlerAdapter {
    fn handle(&self, command: Command) -> Result<CommandResponse, CommandError> {
        match command {
            Command::Workflow(command) => self
                .workflow_handler
                .handle(command)
                .map(CommandResponse::Workflow)
                .map_err(CommandError::Workflow),
            Command::Job(command) => self
                .job_handler
                .handle(*command)
                .map(CommandResponse::Job)
                .map_err(CommandError::Job),
            Command::Step(command) => self
                .step_handler
                .handle(command)
                .map(|response| CommandResponse::Step(Box::new(response)))
                .map_err(CommandError::Step),
            Command::Action(command) => self
                .action_handler
                .handle(command)
                .map(CommandResponse::Action)
                .map_err(CommandError::Step),
        }
    }
}
