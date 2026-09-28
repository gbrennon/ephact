use std::{error::Error, time::Instant};

use crate::{
    domain::{
        messages::{
            commands::ExecuteWorkflowCommand,
            events::{
                DomainEvent, RunFailedPayload, RunStartedPayload, WorkflowRunCompletedPayload,
            },
        },
        services::{
            repository_factory::RepositoryFactory,
            workflow_run_config_factory::WorkflowRunConfigFactory,
            workflow_run_config_input::WorkflowRunConfigInput,
        },
    },
    dtos::{
        requests::RunAllWorkflowsRequest,
        responses::{RunSummaryResponse, WorkflowExecutionResponse},
    },
    errors::ApplicationError,
    ports::{
        inbound::run_all_workflows_port::RunAllWorkflowsPort,
        outbound::{
            DetectWorkflowTriggerPort, WorkflowSourcePort,
            domain_event_bus_port::DomainEventBusPort,
            workflow_command_bus_port::WorkflowCommandBusPort,
        },
    },
    services::workflow_execution_aggregator::WorkflowExecutionAggregator,
};

/// Name reported for the aggregate summary of a full multi-workflow run.
pub const ALL_WORKFLOWS_SUMMARY_NAME: &str = "All Workflows";

/// Application service running every workflow found in the repository.
///
/// Reads all workflow sources through an outbound port and publishes one
/// [`ExecuteWorkflowCommand`] per workflow. When every workflow finished, the
/// completion is announced as an [`DomainEvent::WorkflowRunCompleted`] event so
/// infrastructure handlers can clean up.
pub struct RunAllWorkflowsService {
    workflow_source: Box<dyn WorkflowSourcePort>,
    command_bus: Box<dyn WorkflowCommandBusPort>,
    event_bus: Box<dyn DomainEventBusPort>,
    trigger_detector: Box<dyn DetectWorkflowTriggerPort>,
}

impl RunAllWorkflowsService {
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

impl RunAllWorkflowsPort for RunAllWorkflowsService {
    fn execute(
        &self,
        request: RunAllWorkflowsRequest,
    ) -> Result<RunSummaryResponse, ApplicationError> {
        let started_at = Instant::now();
        let repository = RepositoryFactory::create(
            request.repository_path().to_path_buf(),
            request.repository_name().to_string(),
        )
        .map_err(|error| ApplicationError::Workflow(format!("{error:?}")))?;
        let config = Self::create_config(&request);
        let event = Self::required_event(&config)?;
        let run_id = request.run_id().to_string();
        let repository_path = repository.path().as_path().display().to_string();
        self.event_bus
            .publish(DomainEvent::RunStarted(RunStartedPayload::new(
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
        WorkflowRunConfigFactory::create(
            WorkflowRunConfigInput::default()
                .with_workflow(request.workflow().map(str::to_string))
                .with_job(request.job().map(str::to_string))
                .with_event(request.event().map(str::to_string))
                .with_inputs(request.inputs().to_vec())
                .with_secrets(request.secrets().to_vec())
                .with_all_workflows(request.all_workflows())
                .with_allow_repo_writes(request.allow_repo_writes())
                .with_allow_real_container(request.allow_real_container())
                .with_allow_real_fetcher(request.allow_real_fetcher())
                .with_allow_network(request.allow_network()),
        )
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
        self.filter_workflow_contents(workflow_contents, event)
            .into_iter()
            .map(|content| {
                self.command_bus
                    .dispatch(ExecuteWorkflowCommand::new(
                        content,
                        config.clone(),
                        repository.clone(),
                        run_id.to_string(),
                        config.allow_repo_writes(),
                    ))
                    .map_err(|error| ApplicationError::Workflow(error.to_string()))
            })
            .collect()
    }
    fn filter_workflow_contents(&self, workflow_contents: Vec<String>, event: &str) -> Vec<String> {
        workflow_contents
            .into_iter()
            .filter(|content| self.trigger_detector.triggers_on_event(content, event))
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
        self.event_bus.publish(DomainEvent::WorkflowRunCompleted(
            WorkflowRunCompletedPayload::new(
                run_id.to_string(),
                repository_path.to_string(),
                container_names,
                success,
            ),
        ));
    }

    fn announce_run_failed(&self, run_id: &str, repository_path: &str, error: &dyn Error) {
        self.event_bus
            .publish(DomainEvent::RunFailed(RunFailedPayload::new(
                run_id.to_string(),
                repository_path.to_string(),
                None,
                error.to_string(),
            )));
    }
}
