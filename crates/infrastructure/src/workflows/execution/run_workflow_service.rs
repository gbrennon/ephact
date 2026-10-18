use std::{error::Error, future::Future, pin::Pin};

use super::run_execution_context::RunExecutionContext;
use crate::{
    application::{
        dtos::{
            requests::RunWorkflowRequest,
            responses::{RunSummaryResponse, WorkflowExecutionResponse},
        },
        errors::ApplicationError,
        ports::{
            inbound::RunWorkflowPort,
            outbound::{
                DetectWorkflowTriggerPort, WorkflowCommandPublisherPort, WorkflowSourcePort,
                domain_event_publisher_port::DomainEventPublisherPort,
            },
        },
    },
    domain::{
        Validatable,
        messages::{
            commands::ExecuteWorkflowPayload,
            events::{Event, RunFailedPayload, RunStartedPayload, WorkflowRunCompletedPayload},
        },
    },
};
/// Coordinates one workflow run: resolves the repository workflow, verifies
/// that the requested event is declared, dispatches execution, publishes
/// lifecycle events, and returns a run summary or error.
pub struct RunWorkflowService {
    workflow_source: Box<dyn WorkflowSourcePort>,
    command_publisher: Box<dyn WorkflowCommandPublisherPort>,
    event_publisher: Box<dyn DomainEventPublisherPort>,
    trigger_detector: Box<dyn DetectWorkflowTriggerPort>,
}

impl RunWorkflowService {
    pub fn new(
        workflow_source: Box<dyn WorkflowSourcePort>,
        command_publisher: Box<dyn WorkflowCommandPublisherPort>,
        event_publisher: Box<dyn DomainEventPublisherPort>,
        trigger_detector: Box<dyn DetectWorkflowTriggerPort>,
    ) -> Self {
        Self {
            workflow_source,
            command_publisher,
            event_publisher,
            trigger_detector,
        }
    }
}

impl RunWorkflowPort for RunWorkflowService {
    fn execute(
        &self,
        request: RunWorkflowRequest,
    ) -> Pin<Box<dyn Future<Output = Result<RunSummaryResponse, ApplicationError>> + Send + '_>>
    {
        Box::pin(async move {
            let mut context = RunExecutionContext::new(request)
                .map_err(|error| ApplicationError::Workflow(error.to_string()))?;
            self.announce_run_started(&context);
            let workflow_source = self
                .read_workflow(&context)
                .map_err(|error| ApplicationError::Workflow(error.to_string()))?;
            context
                .workflow_name_mut()
                .get_or_insert(workflow_source.file_name().to_owned());
            self.ensure_requested_trigger(&context, workflow_source.content())
                .map_err(|error| ApplicationError::Workflow(error.to_string()))?;
            let execution = self
                .dispatch_workflow(&context, workflow_source)
                .map_err(|error| ApplicationError::Workflow(error.to_string()))?;
            Ok(self.complete_run(&context, execution))
        })
    }
}
impl RunWorkflowService {
    fn announce_run_started(&self, context: &RunExecutionContext) {
        self.event_publisher
            .publish(Event::RunStarted(RunStartedPayload::new(
                context.run_id().to_owned(),
                context.repository_path().to_owned(),
            )));
    }

    fn read_workflow(
        &self,
        context: &RunExecutionContext,
    ) -> Result<crate::application::dtos::responses::WorkflowSourceFileResponse, Box<dyn Error>>
    {
        match self.workflow_source.read_workflow_in_directory(
            context.repository(),
            context
                .config()
                .workflow()
                .map(|workflow| workflow.as_str()),
            context.config().workflow_directory(),
        ) {
            Ok(workflow) => Ok(workflow),
            Err(error) => {
                let error: Box<dyn Error> = Box::new(error);
                self.announce_run_failed(context, error.as_ref());
                Err(error)
            }
        }
    }

    fn ensure_requested_trigger(
        &self,
        context: &RunExecutionContext,
        workflow_content: &str,
    ) -> Result<(), Box<dyn Error>> {
        if !context.config().is_valid() {
            let error: Box<dyn Error> = "workflow event must be specified".into();
            self.announce_run_failed(context, &*error);
            return Err(error);
        }
        let event = context
            .config()
            .event()
            .expect("valid workflow run configuration has an event");
        if self
            .trigger_detector
            .triggers_on_event(workflow_content, event.as_str())
        {
            return Ok(());
        }
        let error: Box<dyn Error> =
            format!("workflow does not define a {} event", event.as_str()).into();
        self.announce_run_failed(context, &*error);
        Err(error)
    }

    fn dispatch_workflow(
        &self,
        context: &RunExecutionContext,
        workflow_source: crate::application::dtos::responses::WorkflowSourceFileResponse,
    ) -> Result<WorkflowExecutionResponse, Box<dyn Error>> {
        let command = ExecuteWorkflowPayload::new(
            workflow_source.content().to_owned(),
            context.config().clone(),
            context.repository().clone(),
            context.run_id().to_owned(),
            context.config().allow_repo_writes(),
        )
        .with_workflow_file_name(workflow_source.file_name());
        match self.command_publisher.publish(command) {
            Ok(execution) => Ok(execution),
            Err(error) => {
                let error: Box<dyn Error> = Box::new(error);
                self.announce_run_failed(context, error.as_ref());
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
        self.event_publisher.publish(Event::WorkflowRunCompleted(
            WorkflowRunCompletedPayload::new(
                context.run_id().to_owned(),
                context.repository_path().to_owned(),
                container_names,
                success,
            ),
        ));
        RunSummaryResponse::new(
            workflow_name,
            job_summaries,
            success,
            context.started_at().elapsed(),
        )
    }
}

impl RunWorkflowService {
    fn announce_run_failed(&self, context: &RunExecutionContext, error: &dyn Error) {
        self.event_publisher
            .publish(Event::RunFailed(RunFailedPayload::new(
                context.run_id().to_owned(),
                context.repository_path().to_owned(),
                context.workflow_name().cloned(),
                error.to_string(),
            )));
    }
}
