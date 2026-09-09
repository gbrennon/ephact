use std::{error::Error, sync::Arc, time::Instant};

use crate::application::{
    commands::ExecuteWorkflowCommand,
    services::pull_request_workflow::{
        config_for_pull_request_event, content_has_pull_request_event,
    },
};
use crate::{
    application::{
        dtos::{JobSummary, RunAllWorkflowsRequest, RunSummary, WorkflowExecution},
        ports::{
            inbound::run_all_workflows_port::RunAllWorkflowsPort,
            outbound::{CommandBusPort, EventBusPort, WorkflowSourcePort},
        },
    },
    domain::events::{ActRunCompletedPayload, DomainEvent},
};

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
}

impl RunAllWorkflowsService {
    pub fn new(
        workflow_source: Box<dyn WorkflowSourcePort>,
        command_bus: Arc<dyn CommandBusPort>,
        event_bus: Arc<dyn EventBusPort>,
    ) -> Self {
        Self {
            workflow_source,
            command_bus,
            event_bus,
        }
    }
}

impl RunAllWorkflowsPort for RunAllWorkflowsService {
    fn execute(&self, request: RunAllWorkflowsRequest) -> Result<RunSummary, Box<dyn Error>> {
        let started_at = Instant::now();
        let executions = self.execute_all_workflows(&request)?;
        let success = executions.iter().all(|execution| execution.success());

        self.announce_run_completed(&executions, success);

        Ok(RunSummary::new(
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
    ) -> Result<Vec<WorkflowExecution>, Box<dyn Error>> {
        let workflow_contents = self
            .workflow_source
            .read_all_workflows(request.repository())?;
        workflow_contents
            .into_iter()
            .filter(|content| content_has_pull_request_event(content))
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

    fn announce_run_completed(&self, executions: &[WorkflowExecution], success: bool) {
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

fn collect_job_summaries(executions: &[WorkflowExecution]) -> Vec<JobSummary> {
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

fn qualified_job_summary(execution: &WorkflowExecution, job: &JobSummary) -> JobSummary {
    let qualified = job
        .name()
        .map(|name| format!("{} / {}", execution.workflow_name(), name));
    job.clone().with_name(qualified)
}
