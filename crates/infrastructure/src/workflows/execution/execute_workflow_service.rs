use std::{collections::HashSet, error::Error};

use crate::{
    application::{
        dtos::{
            requests::{ExecuteWorkflowRequest, LoadWorkflowRequest},
            responses::{
                JobExecutionResponse, JobSummaryResponse, StepSummaryDetails, StepSummaryResponse,
                StepSummaryResponseInput, WorkflowExecutionResponse,
            },
        },
        errors::ExecuteWorkflowError,
        ports::{
            inbound::execute_workflow_port::ExecuteWorkflowPort,
            outbound::{
                JobCommandPublisherPort, domain_event_publisher_port::DomainEventPublisherPort,
                workflow_loader_port::WorkflowLoaderPort,
            },
        },
    },
    domain::{
        entities::JobRun,
        messages::{
            commands::ExecuteJobPayload,
            events::{Event, JobFinishedPayload, JobStartedPayload, WorkflowStartedPayload},
        },
        value_objects::EvaluationContext,
    },
};

/// Application service coordinating the execution of a single workflow.
///
/// Loads the workflow definition through an outbound port, plans its job
/// stages, and publishes one [`ExecuteJobPayload`] per planned run. The job
/// command handler is what turns each command into an execution, so this
/// service never depends on the job entrypoint itself. Progress facts are
/// announced as domain events on the outbound [`DomainEventPublisherPort`].
pub struct ExecuteWorkflowService {
    workflow_loader: Box<dyn WorkflowLoaderPort>,
    command_publisher: Box<dyn JobCommandPublisherPort>,
    event_publisher: Box<dyn DomainEventPublisherPort>,
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
        command_publisher: Box<dyn JobCommandPublisherPort>,
        event_publisher: Box<dyn DomainEventPublisherPort>,
    ) -> Self {
        Self {
            workflow_loader,
            command_publisher,
            event_publisher,
        }
    }
}

impl ExecuteWorkflowPort for ExecuteWorkflowService {
    fn execute(
        &self,
        request: ExecuteWorkflowRequest,
    ) -> Result<WorkflowExecutionResponse, ExecuteWorkflowError> {
        let context = request.context().clone();
        let workflow = self
            .workflow_loader
            .load(LoadWorkflowRequest::new(
                request.workflow_content().to_string(),
                request.file_name().unwrap_or("unnamed").to_string(),
            ))
            .map_err(|error| ExecuteWorkflowError::Workflow(error.to_string()))?;
        let workflow_name = workflow.name().or(workflow.file()).unwrap_or("unnamed");
        let plan = workflow
            .plan()
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
    ) -> Result<Vec<JobExecutionResponse>, Box<dyn Error>> {
        let mut blocked_jobs = HashSet::new();
        let mut executions = Vec::new();
        for run in plan.stages().iter().flat_map(|stage| stage.runs()) {
            let input = JobExecutionInput::new(workflow, run, &request, context);
            executions.push(self.execute_planned_run(run, input, &mut blocked_jobs)?);
        }
        Ok(executions)
    }

    fn execute_planned_run(
        &self,
        run: &JobRun,
        input: JobExecutionInput<'_>,
        blocked_jobs: &mut HashSet<String>,
    ) -> Result<JobExecutionResponse, Box<dyn Error>> {
        if Self::run_is_blocked(run, blocked_jobs) {
            blocked_jobs.insert(run.job_id().to_string());
            return Ok(Self::skipped_run(run));
        }
        let execution = self.execute_run(input)?;
        if !execution.job_summary().success() && !run.job().continues_after_failure() {
            blocked_jobs.insert(run.job_id().to_string());
        }
        Ok(execution)
    }

    fn run_is_blocked(run: &JobRun, blocked_jobs: &HashSet<String>) -> bool {
        run.job()
            .needs()
            .iter()
            .any(|dependency| blocked_jobs.contains(dependency))
    }

    fn skipped_run(run: &JobRun) -> JobExecutionResponse {
        let steps = run
            .job()
            .steps()
            .iter()
            .map(|step| {
                StepSummaryResponse::new(StepSummaryResponseInput::new(
                    step.display_name(),
                    step.step_type(),
                    StepSummaryDetails::new(
                        None,
                        step.continues_on_error(),
                        std::time::Duration::ZERO,
                        "",
                        "",
                    ),
                ))
                .with_skip_reason("dependency failed")
            })
            .collect();
        JobExecutionResponse::new(
            JobSummaryResponse::new(
                run.job_id().to_string(),
                run.job().name().map(str::to_string),
                steps,
                false,
            )
            .with_skip_reason("dependency failed"),
            "",
        )
    }

    fn execute_run(
        &self,
        input: JobExecutionInput<'_>,
    ) -> Result<crate::application::dtos::responses::JobExecutionResponse, Box<dyn Error>> {
        let workflow_name = input
            .workflow
            .name()
            .or(input.workflow.file())
            .unwrap_or("unnamed");
        self.announce_job_started(workflow_name, input.run);
        let execution = self
            .command_publisher
            .publish(
                ExecuteJobPayload::new(
                    input.run.job().clone(),
                    input.run.job_id().to_string(),
                    input.workflow.clone(),
                    input.repo_path.to_path_buf(),
                    input.context.clone(),
                )
                .with_run_id(input.run_id.to_string())
                .with_allow_repo_writes(input.allow_repo_writes)
                .with_allow_network(input.allow_network),
            )
            .map_err(|error| Box::new(error) as Box<dyn Error>)?;
        self.announce_job_finished(workflow_name, input.run, execution.job_summary().success());
        Ok(execution)
    }

    fn announce_workflow_started(&self, workflow_name: &str) {
        self.event_publisher
            .publish(Event::WorkflowStarted(WorkflowStartedPayload::new(
                workflow_name.to_string(),
            )));
    }

    fn announce_job_started(&self, workflow_name: &str, run: &JobRun) {
        self.event_publisher
            .publish(Event::JobStarted(JobStartedPayload::new(
                workflow_name.to_string(),
                run.job_id().to_string(),
                run.job().name().map(str::to_string),
            )));
    }

    fn announce_job_finished(&self, workflow_name: &str, run: &JobRun, job_success: bool) {
        self.event_publisher
            .publish(Event::JobFinished(JobFinishedPayload::new(
                workflow_name.to_string(),
                run.job_id().to_string(),
                run.job().name().map(str::to_string),
                job_success,
            )));
    }
}
