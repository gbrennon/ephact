use std::{error::Error, sync::Arc};

use crate::application::commands::ExecuteJobCommand;
use crate::application::dtos::requests::ExecuteWorkflowRequest;
use crate::application::dtos::requests::LoadWorkflowRequest;
use crate::application::dtos::responses::WorkflowExecutionResponse;
use crate::application::ports::inbound::execute_workflow_port::ExecuteWorkflowPort;
use crate::application::ports::outbound::command_bus_port::CommandBusPort;
use crate::application::ports::outbound::event_bus_port::EventBusPort;
use crate::application::ports::outbound::load_workflow_port::LoadWorkflowPort;
use crate::domain::entities::JobRun;
use crate::domain::events::DomainEvent;
use crate::domain::events::JobFinishedPayload;
use crate::domain::events::JobStartedPayload;
use crate::domain::events::WorkflowStartedPayload;
use crate::domain::services::ExecutionPlanner;

/// Application service coordinating the execution of a single workflow.
///
/// Loads the workflow definition through an outbound port, plans its job
/// stages, and publishes one [`ExecuteJobCommand`] per planned run. The job
/// command handler is what turns each command into an execution, so this
/// service never depends on the job entrypoint itself. Progress facts are
/// announced as domain events on the outbound [`EventBusPort`].
pub struct ExecuteWorkflowService {
    workflow_loader: Box<dyn LoadWorkflowPort>,
    command_bus: Arc<dyn CommandBusPort>,
    event_bus: Arc<dyn EventBusPort>,
}

impl ExecuteWorkflowService {
    pub fn new(
        workflow_loader: Box<dyn LoadWorkflowPort>,
        command_bus: Arc<dyn CommandBusPort>,
        event_bus: Arc<dyn EventBusPort>,
    ) -> Self {
        Self {
            workflow_loader,
            command_bus,
            event_bus,
        }
    }
}

impl ExecuteWorkflowPort for ExecuteWorkflowService {
    fn execute(
        &self,
        request: ExecuteWorkflowRequest<'_>,
    ) -> Result<WorkflowExecutionResponse, Box<dyn Error>> {
        let workflow = self
            .workflow_loader
            .execute(LoadWorkflowRequest::new(request.workflow_content()))?;
        let workflow_name = workflow.name().unwrap_or("unnamed");
        let plan = ExecutionPlanner
            .plan(&workflow)
            .map_err(|e| format!("{:?}", e))?;

        self.announce_workflow_started(workflow_name);

        let executions = self.execute_planned_runs(&workflow, &plan, request)?;

        let job_summaries = executions.iter().map(|e| e.job_summary().clone()).collect();
        let container_names = executions
            .iter()
            .map(|e| e.container_name().to_string())
            .collect();
        let success = executions.iter().all(|e| e.job_summary().success());

        Ok(WorkflowExecutionResponse::new(
            workflow_name,
            job_summaries,
            container_names,
            success,
        ))
    }
}

impl ExecuteWorkflowService {
    fn execute_planned_runs(
        &self,
        workflow: &crate::domain::aggregates::Workflow,
        plan: &crate::domain::value_objects::ExecutionPlan,
        request: ExecuteWorkflowRequest<'_>,
    ) -> Result<Vec<crate::application::dtos::responses::JobExecutionResponse>, Box<dyn Error>>
    {
        let all_runs: Vec<&crate::domain::entities::JobRun> = plan
            .stages()
            .iter()
            .flat_map(|stage| stage.runs().iter())
            .collect();
        all_runs
            .iter()
            .map(|run| self.execute_run(workflow, run, request.repo_path(), request.context()))
            .collect()
    }

    fn execute_run(
        &self,
        workflow: &crate::domain::aggregates::Workflow,
        run: &crate::domain::entities::JobRun,
        repo_path: &std::path::Path,
        context: &crate::domain::value_objects::EvaluationContext,
    ) -> Result<crate::application::dtos::responses::JobExecutionResponse, Box<dyn Error>> {
        self.announce_job_started(workflow.name().unwrap_or("unnamed"), run);
        let execution = self.command_bus.dispatch_job(ExecuteJobCommand::new(
            run.job().clone(),
            run.job_id().to_string(),
            workflow.clone(),
            repo_path.to_path_buf(),
            context.clone(),
        ))?;
        self.announce_job_finished(
            workflow.name().unwrap_or("unnamed"),
            run,
            execution.job_summary().success(),
        );
        Ok(execution)
    }

    fn announce_workflow_started(&self, workflow_name: &str) {
        self.event_bus
            .publish(DomainEvent::WorkflowStarted(WorkflowStartedPayload::new(
                workflow_name.to_string(),
            )));
    }

    fn announce_job_started(&self, workflow_name: &str, run: &JobRun) {
        self.event_bus
            .publish(DomainEvent::JobStarted(JobStartedPayload::new(
                workflow_name.to_string(),
                run.job_id().to_string(),
                run.job().name().map(str::to_string),
            )));
    }

    fn announce_job_finished(&self, workflow_name: &str, run: &JobRun, job_success: bool) {
        self.event_bus
            .publish(DomainEvent::JobFinished(JobFinishedPayload::new(
                workflow_name.to_string(),
                run.job_id().to_string(),
                run.job().name().map(str::to_string),
                job_success,
            )));
    }
}
