use std::{error::Error, sync::Arc};

use crate::application::commands::ExecuteJobCommand;
use crate::{
    application::{
        dtos::{ExecuteWorkflowRequest, LoadWorkflowRequest, WorkflowExecution},
        ports::{
            inbound::execute_workflow_port::ExecuteWorkflowPort,
            outbound::{
                command_bus_port::CommandBusPort, event_bus_port::EventBusPort,
                load_workflow_port::LoadWorkflowPort,
            },
        },
    },
    domain::{
        events::{DomainEvent, JobFinishedPayload, JobStartedPayload, WorkflowStartedPayload},
        planner::{Planner, Run},
    },
};

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
    ) -> Result<WorkflowExecution, Box<dyn Error>> {
        let workflow = self.workflow_loader.execute(LoadWorkflowRequest::new(request.workflow_content()))?;
        let workflow_name = workflow.name().clone().unwrap_or_else(|| "unnamed".into());
        let plan = Planner.plan(&workflow).map_err(|e| format!("{:?}", e))?;

        self.announce_workflow_started(&workflow_name);

        let executions = self.execute_planned_runs(&workflow, &plan, request)?;

        let job_summaries = executions
            .iter()
            .map(|e| e.job_summary().clone())
            .collect();
        let container_names = executions
            .iter()
            .map(|e| e.container_name().to_string())
            .collect();
        let success = executions.iter().all(|e| e.job_summary().success());

        Ok(WorkflowExecution::new(
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
        workflow: &crate::domain::workflow::Workflow,
        plan: &crate::domain::planner::Plan,
        request: ExecuteWorkflowRequest<'_>,
    ) -> Result<Vec<crate::application::dtos::JobExecution>, Box<dyn Error>> {
        let all_runs: Vec<&crate::domain::planner::Run> = plan
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
        workflow: &crate::domain::workflow::Workflow,
        run: &crate::domain::planner::Run,
        repo_path: &std::path::Path,
        context: &crate::domain::expression::EvalContext,
    ) -> Result<crate::application::dtos::JobExecution, Box<dyn Error>> {
        self.announce_job_started(
            workflow.name().unwrap_or("unnamed"),
            run,
        );
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

    fn announce_job_started(&self, workflow_name: &str, run: &Run) {
        self.event_bus
            .publish(DomainEvent::JobStarted(JobStartedPayload::new(
                workflow_name.to_string(),
                run.job_id().to_string(),
                run.job().name().map(str::to_string),
            )));
    }

    fn announce_job_finished(&self, workflow_name: &str, run: &Run, job_success: bool) {
        self.event_bus
            .publish(DomainEvent::JobFinished(JobFinishedPayload::new(
                workflow_name.to_string(),
                run.job_id().to_string(),
                run.job().name().map(str::to_string),
                job_success,
            )));
    }
}
