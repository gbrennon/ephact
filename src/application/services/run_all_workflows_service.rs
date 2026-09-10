use std::{error::Error, sync::Arc, time::Instant};

use crate::application::commands::ExecuteWorkflowCommand;
use crate::application::dtos::requests::RunAllWorkflowsRequest;
use crate::application::dtos::responses::JobSummaryResponse;
use crate::application::dtos::responses::RunSummaryResponse;
use crate::application::dtos::responses::WorkflowExecutionResponse;
use crate::application::ports::inbound::run_all_workflows_port::RunAllWorkflowsPort;
use crate::application::ports::outbound::CommandBusPort;
use crate::application::ports::outbound::DetectWorkflowTriggerPort;
use crate::application::ports::outbound::EventBusPort;
use crate::application::ports::outbound::WorkflowSourcePort;
use crate::application::services::pull_request_workflow::PULL_REQUEST_EVENT_NAME;
use crate::application::services::pull_request_workflow::config_for_pull_request_event;
use crate::domain::events::ActRunCompletedPayload;
use crate::domain::events::DomainEvent;

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
    command_bus: Arc<dyn CommandBusPort>,
    event_bus: Arc<dyn EventBusPort>,
    trigger_detector: Arc<dyn DetectWorkflowTriggerPort>,
}

impl RunAllWorkflowsService {
    pub fn new(
        workflow_source: Box<dyn WorkflowSourcePort>,
        command_bus: Arc<dyn CommandBusPort>,
        event_bus: Arc<dyn EventBusPort>,
        trigger_detector: Arc<dyn DetectWorkflowTriggerPort>,
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
        let executions = self.execute_all_workflows(&request)?;
        let success = executions.iter().all(|execution| execution.success());

        self.announce_run_completed(&executions, success);

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
                self.command_bus
                    .dispatch_workflow(ExecuteWorkflowCommand::new(
                        content,
                        config_for_pull_request_event(request.config().clone()),
                        request.repository().clone(),
                    ))
            })
            .collect()
    }

    fn announce_run_completed(&self, executions: &[WorkflowExecutionResponse], success: bool) {
        let container_names: Vec<String> = executions
            .iter()
            .flat_map(|execution| execution.container_names().to_vec())
            .collect();
        self.event_bus
            .publish(DomainEvent::ActRunCompleted(ActRunCompletedPayload::new(
                container_names,
                success,
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
