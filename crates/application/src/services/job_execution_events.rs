use crate::{
    domain::{
        aggregates::Workflow,
        entities::{JobRun, Step},
        messages::events::{
            ContainerStartedPayload, Event, StepFinishedDetails, StepFinishedPayload,
            StepStartedPayload,
        },
    },
    dtos::{
        requests::ExecuteJobRequest,
        responses::{PreparedJobContainerResponse, SummarizedStepResponse},
    },
    ports::outbound::DomainEventPublisherPort,
};

/// Publishes progress events for one job execution.
pub struct JobExecutionEvents<'a> {
    event_bus: &'a dyn DomainEventPublisherPort,
}

impl<'a> JobExecutionEvents<'a> {
    /// Creates an event publisher for one job execution.
    pub fn new(event_bus: &'a dyn DomainEventPublisherPort) -> Self {
        Self { event_bus }
    }

    /// Announces the container prepared for a job run.
    pub fn publish_container_started(
        &self,
        request: &ExecuteJobRequest,
        prepared: &PreparedJobContainerResponse,
    ) {
        self.event_bus
            .publish(Event::ContainerStarted(ContainerStartedPayload::new(
                request.run_id().to_string(),
                prepared.container_name().to_string(),
            )));
    }

    /// Announces the start of a job step.
    pub fn publish_step_started(&self, workflow: &Workflow, run: &JobRun, step: &Step) {
        self.event_bus
            .publish(Event::StepStarted(StepStartedPayload::new(
                workflow
                    .name()
                    .or(workflow.file())
                    .unwrap_or("unnamed")
                    .to_string(),
                run.job_id().to_string(),
                step.display_name().to_string(),
            )));
    }

    /// Announces the completed summary of a job step.
    pub fn publish_step_finished(
        &self,
        request: &ExecuteJobRequest,
        workflow: &Workflow,
        run: &JobRun,
        summarized: &SummarizedStepResponse,
    ) {
        let fails_job = summarized.fails_job();
        self.event_bus
            .publish(Event::StepFinished(StepFinishedPayload::new(
                request.run_id().to_string(),
                StepFinishedDetails::new(
                    workflow
                        .name()
                        .or(workflow.file())
                        .unwrap_or("unnamed")
                        .to_string(),
                    run.job_id().to_string(),
                    summarized.summary().name().to_string(),
                    !fails_job,
                    summarized.summary().exit_code(),
                )
                .with_stdout(summarized.summary().stdout().to_string())
                .with_stderr(summarized.summary().stderr().to_string()),
            )));
    }
}
