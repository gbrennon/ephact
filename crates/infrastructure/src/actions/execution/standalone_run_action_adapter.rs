use std::{
    process,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::SystemTime,
};

use ephact_application::{
    dtos::{
        requests::{PrepareJobContainerRequest, RunActionRequest},
        responses::ExecuteActionResponse,
    },
    errors::RunActionError,
    ports::{
        inbound::RunActionPort,
        outbound::{DomainEventPublisherPort, JobContainerPreparerPort},
    },
};
use ephact_domain::{
    errors::StepError,
    messages::events::{
        ContainerStartedPayload, Event, RunFailedPayload, WorkflowRunCompletedPayload,
    },
};

use super::RunActionFactory;

/// Runs a standalone action in a prepared job container and emits the same
/// lifecycle events used by workflow execution for cleanup and diagnostics.
pub struct StandaloneRunActionAdapter {
    preparer: Box<dyn JobContainerPreparerPort>,
    action_factory: RunActionFactory,
    events: Arc<dyn DomainEventPublisherPort>,
}

static NEXT_ACTION_RUN_ID: AtomicU64 = AtomicU64::new(0);

impl StandaloneRunActionAdapter {
    pub fn new(
        preparer: Box<dyn JobContainerPreparerPort>,
        action_factory: RunActionFactory,
        events: Arc<dyn DomainEventPublisherPort>,
    ) -> Self {
        Self {
            preparer,
            action_factory,
            events,
        }
    }

    fn run_id() -> String {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_or(0, |duration| duration.as_nanos());
        let sequence = NEXT_ACTION_RUN_ID.fetch_add(1, Ordering::Relaxed);
        format!("run-{}-{timestamp}-{sequence}", process::id())
    }
}

impl RunActionPort for StandaloneRunActionAdapter {
    fn execute(&self, request: RunActionRequest) -> Result<ExecuteActionResponse, RunActionError> {
        let run_id = Self::run_id();
        let repository_path = request.repo_path().to_path_buf();
        let prepared = self
            .preparer
            .prepare(PrepareJobContainerRequest::new(
                run_id.clone(),
                None,
                repository_path.clone(),
                false,
            ))
            .map_err(|error| RunActionError::Step(StepError::new(error.to_string())))?;
        let container_name = prepared.container_name().to_owned();
        let container = prepared.container_handle();
        self.events
            .publish(Event::ContainerStarted(ContainerStartedPayload::new(
                run_id.clone(),
                container_name.clone(),
            )));

        let result = (self.action_factory)(container).execute(request);
        match &result {
            Ok(response) => self.events.publish(Event::WorkflowRunCompleted(
                WorkflowRunCompletedPayload::new(
                    run_id,
                    repository_path.display().to_string(),
                    vec![container_name],
                    response.exit_code() == 0,
                ),
            )),
            Err(error) => self.events.publish(Event::RunFailed(RunFailedPayload::new(
                run_id,
                repository_path.display().to_string(),
                None,
                error.to_string(),
            ))),
        }
        result
    }
}
