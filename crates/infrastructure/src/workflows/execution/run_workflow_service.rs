use std::{error::Error, future::Future, pin::Pin, time::Instant};

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
        Repository, Validatable,
        messages::{
            commands::{Event, ExecuteWorkflowCommand},
            events::{RunFailedPayload, RunStartedPayload, WorkflowRunCompletedPayload},
        },
        value_objects::workflow_run_config::WorkflowRunConfig,
    },
    repositories::RepositoryResolver,
    workflows::execution::workflow_run_config_mapper::WorkflowRunConfigMapper,
};
///
/// Agnostic by construction: it never touches files, containers, or any external
/// service. It reads the workflow definition through the outbound
/// [`WorkflowSourcePort`], expresses the intent to execute it as a command on the
/// outbound [`WorkflowCommandPublisherPort`], and announces the outcome as a domain event on the
/// outbound [`DomainEventPublisherPort`].
pub struct RunWorkflowService {
    workflow_source: Box<dyn WorkflowSourcePort>,
    command_publisher: Box<dyn WorkflowCommandPublisherPort>,
    event_publisher: Box<dyn DomainEventPublisherPort>,
    trigger_detector: Box<dyn DetectWorkflowTriggerPort>,
}

struct RunExecutionContext {
    repository: Repository,
    repository_path: String,
    config: WorkflowRunConfig,
    run_id: String,
    workflow_name: Option<String>,
    started_at: Instant,
}

impl RunExecutionContext {
    fn new(request: RunWorkflowRequest) -> Result<Self, Box<dyn Error>> {
        let repository_path = request.repository_path().to_path_buf();
        let repository = RepositoryResolver::resolve(
            repository_path.clone(),
            request.repository_name().to_owned(),
        )
        .map_err(|error| format!("{error}"))?;
        let config = WorkflowRunConfigMapper::default()
            .with_workflow(request.workflow().map(str::to_string))
            .with_job(request.job().map(str::to_string))
            .with_event(request.event().map(str::to_string))
            .with_inputs(request.inputs().to_vec())
            .with_secrets(request.secrets().to_vec())
            .with_all_workflows(request.all_workflows())
            .with_allow_repo_writes(request.allow_repo_writes())
            .with_allow_real_container(request.allow_real_container())
            .with_allow_real_fetcher(request.allow_real_fetcher())
            .with_allow_network(request.allow_network())
            .into_config();
        let run_id = request.run_id().to_string();
        let workflow_name = config
            .workflow()
            .map(|workflow| workflow.as_str().to_string());
        Ok(Self {
            repository,
            repository_path: repository_path.display().to_string(),
            config,
            run_id,
            workflow_name,
            started_at: Instant::now(),
        })
    }
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
                .workflow_name
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
                context.run_id.clone(),
                context.repository_path.clone(),
            )));
    }

    fn read_workflow(
        &self,
        context: &RunExecutionContext,
    ) -> Result<crate::application::dtos::responses::WorkflowSourceFileResponse, Box<dyn Error>>
    {
        match self.workflow_source.read_workflow(
            &context.repository,
            context.config.workflow().map(|workflow| workflow.as_str()),
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
        if !context.config.is_valid() {
            let error: Box<dyn Error> = "workflow event must be specified".into();
            self.announce_run_failed(context, &*error);
            return Err(error);
        }
        let event = context
            .config
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
        let command = ExecuteWorkflowCommand::new(
            workflow_source.content().to_owned(),
            context.config.clone(),
            context.repository.clone(),
            context.run_id.clone(),
            context.config.allow_repo_writes(),
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
                context.run_id.clone(),
                context.repository_path.clone(),
                container_names,
                success,
            ),
        ));
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
        self.event_publisher
            .publish(Event::RunFailed(RunFailedPayload::new(
                context.run_id.clone(),
                context.repository_path.clone(),
                context.workflow_name.clone(),
                error.to_string(),
            )));
    }
}
