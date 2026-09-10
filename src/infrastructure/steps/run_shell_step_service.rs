use std::sync::Arc;

use crate::application::dtos::ExecResult;
use crate::application::dtos::RunShellStepRequest;
use crate::application::ports::outbound::event_bus_port::EventBusPort;
use crate::application::ports::outbound::run_shell_step_port::RunShellStepPort;
use crate::domain::errors::StepError;
use crate::domain::events::{DomainEvent, OutputStream, StepOutputPayload};
use crate::domain::value_objects::ShellCommand;

/// Service that runs a step's shell script inside the container it was given,
/// relaying the step's output as [`DomainEvent::StepOutput`] events while it
/// runs.
pub struct RunShellStepService {
    event_bus: Arc<dyn EventBusPort>,
}

impl RunShellStepService {
    pub fn new(event_bus: Arc<dyn EventBusPort>) -> Self {
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
    fn execute(&self, request: RunShellStepRequest<'_>) -> Result<ExecResult, StepError> {
        let command = ShellCommand::for_step(request.step(), request.env())
            .ok_or_else(|| StepError::new("step has neither `run` nor `uses` defined"))?;
        let step_name = request.step().display_name();

        let mut relay = |stream: OutputStream, text: &str| {
            self.relay_output(step_name, stream, text);
        };

        request
            .container()
            .exec_streaming(
                command.argv(),
                command.working_directory(),
                command.env(),
                &mut relay,
            )
            .map_err(|error| StepError::new(format!("{error:?}")))
    }
}
