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
use crate::domain::messages::commands::ExecuteWorkflowCommand;
use crate::domain::messages::events::ActRunCompletedPayload;
use crate::domain::messages::events::DomainEvent;

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
        let config = request.into_config();
        let started_at = Instant::now();

        let workflow_content = self
            .workflow_source
            .read_workflow(&repository, config.workflow().map(|w| w.as_str()))?;
        if !self
            .trigger_detector
            .triggers_on_event(&workflow_content, PULL_REQUEST_EVENT_NAME)
        {
            return Err("workflow does not define a pull_request event".into());
        }
        let config = config_for_pull_request_event(config);

        let execution = self.command_bus.dispatch(ExecuteWorkflowCommand::new(
            workflow_content,
            config,
            repository,
        ))?;

        let (workflow_name, job_summaries, container_names, success) = execution.into_parts();

        self.event_bus
            .publish(DomainEvent::ActRunCompleted(ActRunCompletedPayload::new(
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
