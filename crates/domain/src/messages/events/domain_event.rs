use super::{
    ContainerStartedPayload, Event, JobFinishedPayload, JobStartedPayload, RunFailedPayload,
    RunStartedPayload, StepFinishedPayload, StepOutputPayload, StepStartedPayload,
    WorkflowRunCompletedPayload, WorkflowStartedPayload,
};

/// Domain events published by application services.
///
/// Events state a fact about something that already happened. They are
/// dispatched in-memory through the application's event bus outbound port, and
/// infrastructure handlers subscribe to the variants they react to (e.g.
/// container cleanup and terminal progress reporting).
#[derive(Debug, Clone)]
pub enum DomainEvent {
    RunStarted(RunStartedPayload),
    RunFailed(RunFailedPayload),
    WorkflowRunCompleted(WorkflowRunCompletedPayload),
    ContainerStarted(ContainerStartedPayload),
    WorkflowStarted(WorkflowStartedPayload),
    JobStarted(JobStartedPayload),
    StepStarted(StepStartedPayload),
    StepOutput(StepOutputPayload),
    StepFinished(StepFinishedPayload),
    JobFinished(JobFinishedPayload),
}

impl Event for DomainEvent {}
