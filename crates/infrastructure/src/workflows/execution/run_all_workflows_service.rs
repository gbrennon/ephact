use std::{error::Error, time::Instant};

use crate::{
    application::{
        dtos::{
            requests::RunAllWorkflowsRequest,
            responses::{RunSummaryResponse, WorkflowExecutionResponse},
        },
        errors::ApplicationError,
        ports::{
            inbound::run_all_workflows_port::RunAllWorkflowsPort,
            outbound::{
                DetectWorkflowTriggerPort, WorkflowCommandPublisherPort, WorkflowSourcePort,
                domain_event_publisher_port::DomainEventPublisherPort,
            },
        },
    },
    domain::messages::{
        commands::ExecuteWorkflowPayload,
        events::{Event, RunFailedPayload, RunStartedPayload, WorkflowRunCompletedPayload},
    },
    repositories::RepositoryResolver,
    workflows::execution::{
        workflow_execution_aggregator::WorkflowExecutionAggregator,
        workflow_run_config_mapper::WorkflowRunConfigMapper,
    },
};

/// Name reported for the aggregate summary of a full multi-workflow run.
pub const ALL_WORKFLOWS_SUMMARY_NAME: &str = "All Workflows";

/// Runs every repository workflow that declares the requested event, aggregates
/// the workflow results, announces completion, and returns an overall summary.
pub struct RunAllWorkflowsService {
    workflow_source: Box<dyn WorkflowSourcePort>,
    command_publisher: Box<dyn WorkflowCommandPublisherPort>,
    event_publisher: Box<dyn DomainEventPublisherPort>,
    trigger_detector: Box<dyn DetectWorkflowTriggerPort>,
}

impl RunAllWorkflowsService {
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

impl RunAllWorkflowsPort for RunAllWorkflowsService {
    fn execute(
        &self,
        request: RunAllWorkflowsRequest,
    ) -> Result<RunSummaryResponse, ApplicationError> {
        let started_at = Instant::now();
        let repository = RepositoryResolver::resolve(
            request.repository_path().to_path_buf(),
            request.repository_name().to_owned(),
        )
        .map_err(|error| ApplicationError::Workflow(error.to_string()))?;
        let config = Self::create_config(&request);
        let event = Self::required_event(&config)?;
        let repository_path = repository.path().as_path().display().to_string();
        let run_id = request.run_id().to_string();
        self.event_publisher
            .publish(Event::RunStarted(RunStartedPayload::new(
                run_id.clone(),
                repository_path.clone(),
            )));
        let executions = match self.execute_all_workflows(&repository, &config, event, &run_id) {
            Ok(executions) => executions,
            Err(error) => {
                let error = ApplicationError::Workflow(error.to_string());
                self.announce_run_failed(&run_id, &repository_path, &error);
                return Err(error);
            }
        };
        let success = executions.iter().all(|execution| execution.success());

        self.announce_run_completed(&run_id, &repository_path, &executions, success);

        Ok(RunSummaryResponse::new(
            ALL_WORKFLOWS_SUMMARY_NAME,
            WorkflowExecutionAggregator::new().aggregate(&executions),
            success,
            started_at.elapsed(),
        ))
    }
}
impl RunAllWorkflowsService {
    fn create_config(
        request: &RunAllWorkflowsRequest,
    ) -> crate::domain::value_objects::WorkflowRunConfig {
        WorkflowRunConfigMapper::default()
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
            .into_config()
    }

    fn required_event(
        config: &crate::domain::value_objects::WorkflowRunConfig,
    ) -> Result<&str, ApplicationError> {
        config.event().map(|event| event.as_str()).ok_or_else(|| {
            ApplicationError::Workflow("workflow event must be specified".to_owned())
        })
    }

    fn execute_all_workflows(
        &self,
        repository: &crate::domain::Repository,
        config: &crate::domain::value_objects::WorkflowRunConfig,
        event: &str,
        run_id: &str,
    ) -> Result<Vec<WorkflowExecutionResponse>, ApplicationError> {
        let workflow_contents = self
            .workflow_source
            .read_all_workflows(repository)
            .map_err(|error| ApplicationError::Workflow(error.to_string()))?;
        self.filter_workflow_sources(workflow_contents, event)
            .into_iter()
            .map(|workflow| {
                let command = ExecuteWorkflowPayload::new(
                    workflow.content().to_owned(),
                    config.clone(),
                    repository.clone(),
                    run_id.to_string(),
                    config.allow_repo_writes(),
                )
                .with_workflow_file_name(workflow.file_name());
                self.command_publisher
                    .publish(command)
                    .map_err(|error| ApplicationError::Workflow(error.to_string()))
            })
            .collect()
    }
    fn filter_workflow_sources(
        &self,
        workflow_sources: Vec<crate::application::dtos::responses::WorkflowSourceFileResponse>,
        event: &str,
    ) -> Vec<crate::application::dtos::responses::WorkflowSourceFileResponse> {
        workflow_sources
            .into_iter()
            .filter(|workflow| {
                self.trigger_detector
                    .triggers_on_event(workflow.content(), event)
            })
            .collect()
    }

    fn announce_run_completed(
        &self,
        run_id: &str,
        repository_path: &str,
        executions: &[WorkflowExecutionResponse],
        success: bool,
    ) {
        let container_names: Vec<String> = executions
            .iter()
            .flat_map(|execution| execution.container_names().to_vec())
            .collect();
        self.event_publisher.publish(Event::WorkflowRunCompleted(
            WorkflowRunCompletedPayload::new(
                run_id.to_string(),
                repository_path.to_string(),
                container_names,
                success,
            ),
        ));
    }

    fn announce_run_failed(&self, run_id: &str, repository_path: &str, error: &dyn Error) {
        self.event_publisher
            .publish(Event::RunFailed(RunFailedPayload::new(
                run_id.to_string(),
                repository_path.to_string(),
                None,
                error.to_string(),
            )));
    }
}
