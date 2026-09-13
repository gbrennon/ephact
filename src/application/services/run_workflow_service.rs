use crate::domain::messages::commands::ExecuteWorkflowCommand;

use std::{error::Error, time::Instant};

use crate::application::dtos::requests::RunWorkflowRequest;
use crate::application::dtos::responses::RunSummaryResponse;
use crate::application::ports::inbound::RunWorkflowPort;
use crate::application::ports::outbound::DetectWorkflowTriggerPort;
use crate::application::ports::outbound::WorkflowSourcePort;
use crate::application::ports::outbound::command_bus_port::WorkflowCommandBusPort;
use crate::application::ports::outbound::event_bus_port::DomainEventBusPort;
use crate::application::services::pull_request_workflow::PULL_REQUEST_EVENT_NAME;
use crate::application::services::pull_request_workflow::config_for_pull_request_event;
use crate::domain::messages::events::ActRunCompletedPayload;
use crate::domain::messages::events::DomainEvent;
use crate::domain::messages::events::RunFailedPayload;
use crate::domain::messages::events::RunStartedPayload;
/// Application service implementing the entrypoint to run a single workflow.
///
/// Agnostic by construction: it never touches files, containers, or any external
/// service. It reads the workflow definition through the outbound
/// [`WorkflowSourcePort`], expresses the intent to execute it as a command on the
/// outbound [`WorkflowCommandBusPort`], and announces the outcome as a domain event on the
/// outbound [`DomainEventBusPort`].
pub struct RunWorkflowService {
    workflow_source: Box<dyn WorkflowSourcePort>,
    command_bus: Box<WorkflowCommandBusPort>,
    event_bus: Box<DomainEventBusPort>,
    trigger_detector: Box<dyn DetectWorkflowTriggerPort>,
}

impl RunWorkflowService {
    pub fn new(
        workflow_source: Box<dyn WorkflowSourcePort>,
        command_bus: Box<WorkflowCommandBusPort>,
        event_bus: Box<DomainEventBusPort>,
        trigger_detector: Box<dyn DetectWorkflowTriggerPort>,
    ) -> Self {
        Self {
            workflow_source,
            command_bus,
            event_bus,
            trigger_detector,
        }
    }
}

impl RunWorkflowPort for RunWorkflowService {
    fn execute(&self, request: RunWorkflowRequest) -> Result<RunSummaryResponse, Box<dyn Error>> {
        let repository = request.repository().clone();
        let repository_path = repository.path().as_path().display().to_string();
        let config = request.into_config();
        let run_id = config.run_id().to_string();
        let workflow_name = config
            .workflow()
            .map(|workflow| workflow.as_str().to_string());
        let started_at = Instant::now();

        self.event_bus
            .publish(DomainEvent::RunStarted(RunStartedPayload::new(
                run_id.clone(),
                repository_path.clone(),
            )));

        let workflow_content = match self
            .workflow_source
            .read_workflow(&repository, config.workflow().map(|w| w.as_str()))
        {
            Ok(content) => content,
            Err(error) => {
                self.announce_run_failed(&run_id, &repository_path, workflow_name.clone(), &*error);
                return Err(error);
            }
        };
        if !self
            .trigger_detector
            .triggers_on_event(&workflow_content, PULL_REQUEST_EVENT_NAME)
        {
            let error: Box<dyn Error> = "workflow does not define a pull_request event".into();
            self.announce_run_failed(&run_id, &repository_path, workflow_name.clone(), &*error);
            return Err(error);
        }
        let config = config_for_pull_request_event(config);

        let execution = match self.command_bus.dispatch(ExecuteWorkflowCommand::new(
            workflow_content,
            config.clone(),
            repository,
            run_id.clone(),
            config.allow_repo_writes(),
        )) {
            Ok(execution) => execution,
            Err(error) => {
                self.announce_run_failed(&run_id, &repository_path, workflow_name, &*error);
                return Err(error);
            }
        };

        let (workflow_name, job_summaries, container_names, success) = execution.into_parts();

        self.event_bus
            .publish(DomainEvent::ActRunCompleted(ActRunCompletedPayload::new(
                run_id,
                repository_path,
                container_names,
                success,
            )));

        Ok(RunSummaryResponse::new(
            workflow_name,
            job_summaries,
            success,
            started_at.elapsed(),
        ))
    }
}
impl RunWorkflowService {
    fn announce_run_failed(
        &self,
        run_id: &str,
        repository_path: &str,
        workflow_name: Option<String>,
        error: &dyn Error,
    ) {
        self.event_bus
            .publish(DomainEvent::RunFailed(RunFailedPayload::new(
                run_id.to_string(),
                repository_path.to_string(),
                workflow_name,
                error.to_string(),
            )));
    }
}
