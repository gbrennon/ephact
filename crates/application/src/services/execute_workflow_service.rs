use std::error::Error;

use crate::{
    domain::{
        entities::JobRun,
        messages::{
            commands::ExecuteJobCommand,
            events::{DomainEvent, JobFinishedPayload, JobStartedPayload, WorkflowStartedPayload},
        },
        services::{ExecutionPlanner, evaluation_context_mapper::EvaluationContextMapper},
        value_objects::EvaluationContext,
    },
    dtos::{
        requests::{ExecuteWorkflowRequest, LoadWorkflowRequest},
        responses::WorkflowExecutionResponse,
    },
    errors::ExecuteWorkflowError,
    ports::{
        inbound::execute_workflow_port::ExecuteWorkflowPort,
        outbound::{
            domain_event_bus_port::DomainEventBusPort, job_command_bus_port::JobCommandBusPort,
            workflow_loader_port::WorkflowLoaderPort,
        },
    },
};

/// Application service coordinating the execution of a single workflow.
///
/// Loads the workflow definition through an outbound port, plans its job
/// stages, and publishes one [`ExecuteJobCommand`] per planned run. The job
/// command handler is what turns each command into an execution, so this
/// service never depends on the job entrypoint itself. Progress facts are
/// announced as domain events on the outbound [`DomainEventBusPort`].
pub struct ExecuteWorkflowService {
    workflow_loader: Box<dyn WorkflowLoaderPort>,
    command_bus: Box<dyn JobCommandBusPort>,
    event_bus: Box<dyn DomainEventBusPort>,
}

struct JobExecutionInput<'a> {
    workflow: &'a crate::domain::aggregates::Workflow,
    run: &'a crate::domain::entities::JobRun,
    repo_path: &'a std::path::Path,
    context: &'a crate::domain::value_objects::EvaluationContext,
    run_id: &'a str,
    allow_repo_writes: bool,
    allow_network: bool,
}

impl<'a> JobExecutionInput<'a> {
    fn new(
        workflow: &'a crate::domain::aggregates::Workflow,
        run: &'a crate::domain::entities::JobRun,
        request: &'a ExecuteWorkflowRequest,
        context: &'a EvaluationContext,
    ) -> Self {
        Self {
            workflow,
            run,
            repo_path: request.repo_path(),
            context,
            run_id: request.run_id(),
            allow_repo_writes: request.allow_repo_writes(),
            allow_network: request.allow_network(),
        }
    }
}

impl ExecuteWorkflowService {
    pub fn new(
        workflow_loader: Box<dyn WorkflowLoaderPort>,
        command_bus: Box<dyn JobCommandBusPort>,
        event_bus: Box<dyn DomainEventBusPort>,
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
        request: ExecuteWorkflowRequest,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError> {
        let context = EvaluationContextMapper::from_parts(request.context().to_vec())
            .map_err(|error| ExecuteWorkflowError::Workflow(format!("{error:?}")))?;
        let workflow = self
            .workflow_loader
            .load(LoadWorkflowRequest::new(
                request.workflow_content().to_string(),
            ))
            .map_err(|error| ExecuteWorkflowError::Workflow(error.to_string()))?;
        let workflow_name = workflow.name().unwrap_or("unnamed");
        let plan = ExecutionPlanner
            .plan(&workflow)
            .map_err(|error| ExecuteWorkflowError::Workflow(format!("{error:?}")))?;

        self.announce_workflow_started(workflow_name);

        let executions = self
            .execute_planned_runs(&workflow, &plan, &context, request)
            .map_err(|error| ExecuteWorkflowError::Workflow(error.to_string()))?;

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
        context: &EvaluationContext,
        request: ExecuteWorkflowRequest,
    ) -> Result<Vec<crate::dtos::responses::JobExecutionResponse>, Box<dyn Error>> {
        let all_runs: Vec<&crate::domain::entities::JobRun> = plan
            .stages()
            .iter()
            .flat_map(|stage| stage.runs().iter())
            .collect();
        all_runs
            .iter()
            .map(|run| self.execute_run(JobExecutionInput::new(workflow, run, &request, context)))
            .collect()
    }

    fn execute_run(
        &self,
        input: JobExecutionInput<'_>,
    ) -> Result<crate::dtos::responses::JobExecutionResponse, Box<dyn Error>> {
        self.announce_job_started(input.workflow.name().unwrap_or("unnamed"), input.run);
        let execution = self.command_bus.dispatch(
            ExecuteJobCommand::new(
                input.run.job().clone(),
                input.run.job_id().to_string(),
                input.workflow.clone(),
                input.repo_path.to_path_buf(),
                input.context.clone(),
            )
            .with_run_id(input.run_id.to_string())
            .with_allow_repo_writes(input.allow_repo_writes)
            .with_allow_network(input.allow_network),
        )?;
        self.announce_job_finished(
            input.workflow.name().unwrap_or("unnamed"),
            input.run,
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
