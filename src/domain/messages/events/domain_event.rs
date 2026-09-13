use super::event::Event;

/// Domain events published by application services.
///
/// Events state a fact about something that already happened. They are
/// dispatched in-memory through the application's event bus outbound port, and
/// infrastructure handlers subscribe to the variants they react to (e.g.
/// container cleanup and terminal progress reporting).
#[derive(Debug, Clone)]
pub enum DomainEvent {
    RunStarted(super::run_started_payload::RunStartedPayload),
    RunFailed(super::run_failed_payload::RunFailedPayload),
    ActRunCompleted(super::act_run_completed_payload::ActRunCompletedPayload),
    WorkflowStarted(super::workflow_started_payload::WorkflowStartedPayload),
    JobStarted(super::job_started_payload::JobStartedPayload),
    StepStarted(super::step_started_payload::StepStartedPayload),
    StepOutput(super::step_output_payload::StepOutputPayload),
    StepFinished(super::step_finished_payload::StepFinishedPayload),
    JobFinished(super::job_finished_payload::JobFinishedPayload),
}

impl Event for DomainEvent {}
