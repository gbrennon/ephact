use crate::{
    application::{
        dtos::{requests::RunShellStepRequest, responses::ExecResultResponse},
        ports::outbound::{
            container_port::ExecOptions, domain_event_bus_port::DomainEventBusPort,
            run_shell_step_port::RunShellStepPort,
        },
    },
    domain::{
        errors::StepError,
        messages::events::{DomainEvent, OutputStream, StepOutputPayload},
        value_objects::ShellCommand,
    },
};

/// Service that runs a step's shell script inside the container it was given,
/// relaying the step's output as [`DomainEvent::StepOutput`] events while it
/// runs.
pub struct RunShellStepService {
    event_bus: Box<dyn DomainEventBusPort>,
}

impl RunShellStepService {
    pub fn new(event_bus: Box<dyn DomainEventBusPort>) -> Self {
        Self { event_bus }
    }

    fn relay_output(&self, step_name: &str, stream: OutputStream, text: &str) {
        self.event_bus
            .publish(DomainEvent::StepOutput(StepOutputPayload::new(
                step_name.to_string(),
                stream,
                text.to_string(),
            )));
    }
}

impl RunShellStepPort for RunShellStepService {
    fn execute(&self, request: RunShellStepRequest<'_>) -> Result<ExecResultResponse, StepError> {
        let command = ShellCommand::for_step(request.step(), request.env())
            .ok_or_else(|| StepError::new("step has neither `run` nor `uses` defined"))?;
        let step_name = request.step().display_name();

        let mut relay = |stream: OutputStream, text: &str| {
            self.relay_output(step_name, stream, text);
        };

        let result = request
            .container()
            .exec_streaming(
                ExecOptions::new(command.argv(), Some("/workspace"), command.env()),
                &mut relay,
            )
            .map_err(|error| StepError::new(format!("{error:?}")))?;

        Ok(ExecResultResponse::new(
            result.exit_code(),
            result.stdout().to_string(),
            result.stderr().to_string(),
        ))
    }
}
