use crate::domain::messages::commands::ExecuteWorkflowCommand;

use std::{error::Error, time::Instant};

use crate::application::dtos::requests::RunAllWorkflowsRequest;
use crate::application::dtos::responses::JobSummaryResponse;
use crate::application::dtos::responses::RunSummaryResponse;
use crate::application::dtos::responses::WorkflowExecutionResponse;
use crate::application::ports::inbound::run_all_workflows_port::RunAllWorkflowsPort;
use crate::application::ports::outbound::DetectWorkflowTriggerPort;
use crate::application::ports::outbound::WorkflowSourcePort;
use crate::application::ports::outbound::domain_event_bus_port::DomainEventBusPort;
use crate::application::ports::outbound::workflow_command_bus_port::WorkflowCommandBusPort;
use crate::application::services::pull_request_workflow::PULL_REQUEST_EVENT_NAME;
use crate::application::services::pull_request_workflow::config_for_pull_request_event;
use crate::domain::messages::events::ActRunCompletedPayload;
use crate::domain::messages::events::DomainEvent;
use crate::domain::messages::events::RunFailedPayload;
use crate::domain::messages::events::RunStartedPayload;

/// Name reported for the aggregate summary of a full multi-workflow run.
pub const ALL_WORKFLOWS_SUMMARY_NAME: &str = "All Workflows";

/// Application service running every workflow found in the repository.
///
/// Reads all workflow sources through an outbound port and publishes one
/// [`ExecuteWorkflowCommand`] per workflow. When every workflow finished, the
/// completion is announced as an [`DomainEvent::ActRunCompleted`] event so
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
    ) -> Result<RunSummaryResponse, Box<dyn Error>> {
        let started_at = Instant::now();
        let run_id = request.config().run_id().to_string();
        let repository_path = request.repository().path().as_path().display().to_string();
        self.event_bus
            .publish(DomainEvent::RunStarted(RunStartedPayload::new(
                run_id.clone(),
                repository_path.clone(),
            )));
        let executions = match self.execute_all_workflows(&request) {
            Ok(executions) => executions,
            Err(error) => {
                self.announce_run_failed(&run_id, &repository_path, &*error);
                return Err(error);
            }
        };
        let success = executions.iter().all(|execution| execution.success());

        self.announce_run_completed(&run_id, &repository_path, &executions, success);

        Ok(RunSummaryResponse::new(
            ALL_WORKFLOWS_SUMMARY_NAME,
            collect_job_summaries(&executions),
            success,
            started_at.elapsed(),
        ))
    }
}

impl RunAllWorkflowsService {
    fn execute_all_workflows(
        &self,
        request: &RunAllWorkflowsRequest,
    ) -> Result<Vec<WorkflowExecutionResponse>, Box<dyn Error>> {
        let workflow_contents = self
            .workflow_source
            .read_all_workflows(request.repository())?;
        workflow_contents
            .into_iter()
            .filter(|content| {
                self.trigger_detector
                    .triggers_on_event(content, PULL_REQUEST_EVENT_NAME)
            })
            .map(|content| {
                self.command_bus.dispatch(ExecuteWorkflowCommand::new(
                    content,
                    config_for_pull_request_event(request.config().clone()),
                    request.repository().clone(),
                    request.config().run_id().to_string(),
                    request.config().allow_repo_writes(),
                ))
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
        self.event_bus
            .publish(DomainEvent::ActRunCompleted(ActRunCompletedPayload::new(
                run_id.to_string(),
                repository_path.to_string(),
                container_names,
                success,
            )));
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

fn collect_job_summaries(executions: &[WorkflowExecutionResponse]) -> Vec<JobSummaryResponse> {
    executions
        .iter()
        .flat_map(|execution| {
            execution
                .job_summaries()
                .iter()
                .map(move |job| qualified_job_summary(execution, job))
        })
        .collect()
}

fn qualified_job_summary(
    execution: &WorkflowExecutionResponse,
    job: &JobSummaryResponse,
) -> JobSummaryResponse {
    let qualified = job
        .name()
        .map(|name| format!("{} / {}", execution.workflow_name(), name));
    job.clone().with_name(qualified)
}
