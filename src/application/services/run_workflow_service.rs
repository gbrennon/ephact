use crate::domain::messages::commands::ExecuteWorkflowCommand;

use std::{error::Error, time::Instant};

use crate::application::dtos::requests::RunWorkflowRequest;
use crate::application::dtos::responses::{RunSummaryResponse, WorkflowExecutionResponse};
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
use crate::domain::{Repository, value_objects::act_run_config::ActRunConfig};
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

struct RunExecutionContext {
    repository: Repository,
    repository_path: String,
    config: ActRunConfig,
    run_id: String,
    workflow_name: Option<String>,
    started_at: Instant,
}

impl RunExecutionContext {
    fn new(request: RunWorkflowRequest) -> Self {
        let repository = request.repository().clone();
        let repository_path = repository.path().as_path().display().to_string();
        let config = request.into_config();
        let run_id = config.run_id().to_string();
        let workflow_name = config
            .workflow()
            .map(|workflow| workflow.as_str().to_string());
        Self {
            repository,
            repository_path,
            config,
            run_id,
            workflow_name,
            started_at: Instant::now(),
        }
    }
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
        let context = RunExecutionContext::new(request);
        self.announce_run_started(&context);
        let workflow_content = self.read_workflow(&context)?;
        self.ensure_pull_request_trigger(&context, &workflow_content)?;
        let execution = self.dispatch_workflow(&context, workflow_content)?;
        Ok(self.complete_run(&context, execution))
    }
}

impl RunWorkflowService {
    fn announce_run_started(&self, context: &RunExecutionContext) {
        self.event_bus
            .publish(DomainEvent::RunStarted(RunStartedPayload::new(
                context.run_id.clone(),
                context.repository_path.clone(),
            )));
    }

    fn read_workflow(&self, context: &RunExecutionContext) -> Result<String, Box<dyn Error>> {
        match self.workflow_source.read_workflow(
            &context.repository,
            context.config.workflow().map(|workflow| workflow.as_str()),
        ) {
            Ok(content) => Ok(content),
            Err(error) => {
                self.announce_run_failed(context, &*error);
                Err(error)
            }
        }
    }

    fn ensure_pull_request_trigger(
        &self,
        context: &RunExecutionContext,
        workflow_content: &str,
    ) -> Result<(), Box<dyn Error>> {
        if self
            .trigger_detector
            .triggers_on_event(workflow_content, PULL_REQUEST_EVENT_NAME)
        {
            return Ok(());
        }
        let error: Box<dyn Error> = "workflow does not define a pull_request event".into();
        self.announce_run_failed(context, &*error);
        Err(error)
    }

    fn dispatch_workflow(
        &self,
        context: &RunExecutionContext,
        workflow_content: String,
    ) -> Result<WorkflowExecutionResponse, Box<dyn Error>> {
        let config = config_for_pull_request_event(context.config.clone());
        match self.command_bus.dispatch(ExecuteWorkflowCommand::new(
            workflow_content,
            config.clone(),
            context.repository.clone(),
            context.run_id.clone(),
            config.allow_repo_writes(),
        )) {
            Ok(execution) => Ok(execution),
            Err(error) => {
                self.announce_run_failed(context, &*error);
                Err(error)
            }
        }
    }

    fn complete_run(
        &self,
        context: &RunExecutionContext,
        execution: WorkflowExecutionResponse,
    ) -> RunSummaryResponse {
        let (workflow_name, job_summaries, container_names, success) = execution.into_parts();
        self.event_bus
            .publish(DomainEvent::ActRunCompleted(ActRunCompletedPayload::new(
                context.run_id.clone(),
                context.repository_path.clone(),
                container_names,
                success,
            )));
        RunSummaryResponse::new(
            workflow_name,
            job_summaries,
            success,
            context.started_at.elapsed(),
        )
    }
}

impl RunWorkflowService {
    fn announce_run_failed(&self, context: &RunExecutionContext, error: &dyn Error) {
        self.event_bus
            .publish(DomainEvent::RunFailed(RunFailedPayload::new(
                context.run_id.clone(),
                context.repository_path.clone(),
                context.workflow_name.clone(),
                error.to_string(),
            )));
    }
}
