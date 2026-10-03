use crate::messages::{
    Message,
    events::{
        ContainerStartedPayload, JobFinishedPayload, JobStartedPayload, RunFailedPayload,
        RunStartedPayload, StepFinishedPayload, StepOutputPayload, StepStartedPayload,
        WorkflowRunCompletedPayload, WorkflowStartedPayload,
    },
};

#[derive(Debug, Clone)]
pub enum Event {
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

impl Message for Event {}
