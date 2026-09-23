use std::{error::Error, future::Future, pin::Pin, time::Instant};

use crate::{
    application::{
        dtos::{
            requests::RunWorkflowRequest,
            responses::{RunSummaryResponse, WorkflowExecutionResponse},
        },
        errors::RunWorkflowError,
        ports::{
            inbound::RunWorkflowPort,
            outbound::{
                DetectWorkflowTriggerPort, WorkflowSourcePort,
                domain_event_bus_port::DomainEventBusPort,
                workflow_command_bus_port::WorkflowCommandBusPort,
            },
        },
    },
    domain::{
        Repository,
        messages::{
            commands::ExecuteWorkflowCommand,
            events::{ActRunCompletedPayload, DomainEvent, RunFailedPayload, RunStartedPayload},
        },
        services::{
            act_run_config_factory::{ActRunConfigFactory, ActRunConfigInput},
            repository_factory::RepositoryFactory,
        },
        value_objects::act_run_config::ActRunConfig,
    },
};
///
/// Agnostic by construction: it never touches files, containers, or any external
/// service. It reads the workflow definition through the outbound
/// [`WorkflowSourcePort`], expresses the intent to execute it as a command on the
/// outbound [`WorkflowCommandBusPort`], and announces the outcome as a domain event on the
/// outbound [`DomainEventBusPort`].
pub struct RunWorkflowService {
    workflow_source: Box<dyn WorkflowSourcePort>,
    command_bus: Box<dyn WorkflowCommandBusPort>,
    event_bus: Box<dyn DomainEventBusPort>,
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
    fn new(request: RunWorkflowRequest) -> Result<Self, Box<dyn Error>> {
        let repository = RepositoryFactory::create(
            request.repository_path().to_path_buf(),
            request.repository_name().to_string(),
        )
        .map_err(|error| format!("{error:?}"))?;
        let repository_path = repository.path().as_path().display().to_string();
        let config = ActRunConfigFactory::create(
            ActRunConfigInput::default()
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
                .with_run_id(request.run_id().to_string()),
        );
        let run_id = config.run_id().to_string();
        let workflow_name = config
            .workflow()
            .map(|workflow| workflow.as_str().to_string());
        Ok(Self {
            repository,
            repository_path,
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
        command_bus: Box<dyn WorkflowCommandBusPort>,
        event_bus: Box<dyn DomainEventBusPort>,
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
    fn execute(
        &self,
        request: RunWorkflowRequest,
    ) -> Pin<Box<dyn Future<Output = Result<RunSummaryResponse, RunWorkflowError>> + Send + '_>>
    {
        Box::pin(async move {
            let context = RunExecutionContext::new(request)
                .map_err(|error| RunWorkflowError::Workflow(error.to_string()))?;
            self.announce_run_started(&context);
            let workflow_content = self
                .read_workflow(&context)
                .map_err(|error| RunWorkflowError::Workflow(error.to_string()))?;
            self.ensure_requested_trigger(&context, &workflow_content)
                .map_err(|error| RunWorkflowError::Workflow(error.to_string()))?;
            let execution = self
                .dispatch_workflow(&context, workflow_content)
                .map_err(|error| RunWorkflowError::Workflow(error.to_string()))?;
            Ok(self.complete_run(&context, execution))
        })
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
        let Some(event) = context.config.event() else {
            let error: Box<dyn Error> = "workflow event must be specified".into();
            self.announce_run_failed(context, &*error);
            return Err(error);
        };
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
        workflow_content: String,
    ) -> Result<WorkflowExecutionResponse, Box<dyn Error>> {
        match self.command_bus.dispatch(ExecuteWorkflowCommand::new(
            workflow_content,
            context.config.clone(),
            context.repository.clone(),
            context.run_id.clone(),
            context.config.allow_repo_writes(),
        )) {
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
