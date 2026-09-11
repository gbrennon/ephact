use super::event::Event;

/// Domain events published by application services.
///
/// Events state a fact about something that already happened. They are
/// dispatched in-memory through the application's event bus outbound port, and
/// infrastructure handlers subscribe to the variants they react to (e.g.
/// container cleanup and terminal progress reporting).
#[derive(Debug, Clone)]
pub enum DomainEvent {
    /// Published when a workflow run completes (success or failure).
    ActRunCompleted(super::act_run_completed_payload::ActRunCompletedPayload),
    /// Published when a workflow's plan is known and its jobs start running.
    WorkflowStarted(super::workflow_started_payload::WorkflowStartedPayload),
    /// Published when one job of a workflow starts running.
    JobStarted(super::job_started_payload::JobStartedPayload),
    /// Published when one step of a job starts running.
    StepStarted(super::step_started_payload::StepStartedPayload),
    /// Published as a running step produces output.
    StepOutput(super::step_output_payload::StepOutputPayload),
    /// Published when one step of a job finishes.
    StepFinished(super::step_finished_payload::StepFinishedPayload),
    /// Published when one job of a workflow finishes.
    JobFinished(super::job_finished_payload::JobFinishedPayload),
}

impl Event for DomainEvent {}
