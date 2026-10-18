use std::{collections::HashSet, error::Error};

use super::job_execution_input::JobExecutionInput;
use crate::{
    application::{
        dtos::{
            requests::ExecuteWorkflowRequest,
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
        aggregates::Workflow,
        entities::JobRun,
        messages::events::{Event, JobFinishedPayload, JobStartedPayload, WorkflowStartedPayload},
    },
    steps::step_interpolator::StepInterpolator,
};

/// Loads and plans one workflow, executes its planned job runs, publishes
/// progress events, and returns their summaries and overall success.
pub struct ExecuteWorkflowService {
    workflow_loader: Box<dyn WorkflowLoaderPort>,
    command_publisher: Box<dyn JobCommandPublisherPort>,
    event_publisher: Box<dyn DomainEventPublisherPort>,
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
        let workflow = self
            .workflow_loader
            .load(
                request.workflow_content(),
                request.file_name().unwrap_or("unnamed"),
            )
            .map_err(|error| ExecuteWorkflowError::Workflow(error.to_string()))?;
        let workflow_name = workflow.name().or(workflow.file()).unwrap_or("unnamed");
        let plan = workflow
            .plan()
            .map_err(|error| ExecuteWorkflowError::Workflow(format!("{error:?}")))?;
        let selected_jobs = Self::selected_job_ids(&workflow, request.selected_job())?;

        self.announce_workflow_started(workflow_name);

        let executions = self
            .execute_planned_runs(&workflow, &plan, selected_jobs.as_ref(), request)
            .map_err(|error| ExecuteWorkflowError::Workflow(error.to_string()))?;

        let job_summaries = executions.iter().map(|e| e.job_summary().clone()).collect();
        let container_names = executions
            .iter()
            .map(|e| e.container_name().to_string())
            .collect();
        let success = executions.iter().all(|e| e.job_summary().success());

        Ok(WorkflowExecutionResponse::new(
            workflow_name.to_owned(),
            job_summaries,
            container_names,
            success,
        ))
    }
}

impl ExecuteWorkflowService {
    fn selected_job_ids(
        workflow: &Workflow,
        selected_job: Option<&str>,
    ) -> Result<Option<HashSet<String>>, ExecuteWorkflowError> {
        let Some(selected_job) = selected_job else {
            return Ok(None);
        };
        Self::validate_selected_job(workflow, selected_job)?;
        Ok(Some(Self::dependency_closure(workflow, selected_job)?))
    }

    fn validate_selected_job(
        workflow: &Workflow,
        selected_job: &str,
    ) -> Result<(), ExecuteWorkflowError> {
        if workflow.job_named(selected_job).is_none() {
            return Err(ExecuteWorkflowError::Workflow(format!(
                "selected job '{selected_job}' was not found"
            )));
        }
        Ok(())
    }

    fn dependency_closure(
        workflow: &Workflow,
        selected_job: &str,
    ) -> Result<HashSet<String>, ExecuteWorkflowError> {
        let mut selected_jobs = HashSet::new();
        let mut pending_jobs = vec![selected_job.to_owned()];
        while let Some(job_id) = pending_jobs.pop() {
            if selected_jobs.insert(job_id.clone()) {
                let Some(job) = workflow.job_named(&job_id) else {
                    return Err(ExecuteWorkflowError::Workflow(format!(
                        "dependency job '{job_id}' was not found"
                    )));
                };
                pending_jobs.extend(job.needs().iter().cloned());
            }
        }
        Ok(selected_jobs)
    }

    fn run_is_selected(run: &JobRun, selected_jobs: Option<&HashSet<String>>) -> bool {
        selected_jobs.is_none_or(|jobs| jobs.contains(run.job_id()))
    }

    fn execute_planned_runs(
        &self,
        workflow: &Workflow,
        plan: &crate::domain::value_objects::ExecutionPlan,
        selected_jobs: Option<&HashSet<String>>,
        request: ExecuteWorkflowRequest,
    ) -> Result<Vec<JobExecutionResponse>, Box<dyn Error>> {
        let mut blocked_jobs = HashSet::new();
        let mut executions = Vec::new();
        for run in plan
            .stages()
            .iter()
            .flat_map(|stage| stage.runs())
            .filter(|run| Self::run_is_selected(run, selected_jobs))
        {
            let input = JobExecutionInput::new(workflow, run, &request, request.context());
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
        self.execute_unblocked_run(run, input, blocked_jobs)
    }

    fn execute_unblocked_run(
        &self,
        run: &JobRun,
        input: JobExecutionInput<'_>,
        blocked_jobs: &mut HashSet<String>,
    ) -> Result<JobExecutionResponse, Box<dyn Error>> {
        if let Some(skipped) = Self::skip_for_false_condition(run, &input, blocked_jobs)? {
            return Ok(skipped);
        }
        let execution = self.execute_run(input)?;
        Self::block_failed_job(run, &execution, blocked_jobs);
        Ok(execution)
    }

    fn block_failed_job(
        run: &JobRun,
        execution: &JobExecutionResponse,
        blocked_jobs: &mut HashSet<String>,
    ) {
        if !execution.job_summary().success() && !run.job().continues_after_failure() {
            blocked_jobs.insert(run.job_id().to_string());
        }
    }

    fn skip_for_false_condition(
        run: &JobRun,
        input: &JobExecutionInput<'_>,
        blocked_jobs: &mut HashSet<String>,
    ) -> Result<Option<JobExecutionResponse>, Box<dyn Error>> {
        let Some(condition) = run.job().if_condition() else {
            return Ok(None);
        };
        let should_run =
            StepInterpolator::evaluate_condition(condition, input.context()).map_err(|error| {
                std::io::Error::other(format!("failed to evaluate job condition: {error:?}"))
            })?;
        if should_run {
            return Ok(None);
        }
        blocked_jobs.insert(run.job_id().to_string());
        Ok(Some(Self::skipped_run_with_reason(
            run,
            "job condition evaluated to false",
        )))
    }

    fn run_is_blocked(run: &JobRun, blocked_jobs: &HashSet<String>) -> bool {
        run.job()
            .needs()
            .iter()
            .any(|dependency| blocked_jobs.contains(dependency))
    }

    fn skipped_run(run: &JobRun) -> JobExecutionResponse {
        Self::skipped_run_with_reason(run, "dependency failed")
    }

    fn skipped_run_with_reason(run: &JobRun, reason: &str) -> JobExecutionResponse {
        let steps = run
            .job()
            .steps()
            .iter()
            .map(|step| {
                StepSummaryResponse::new(StepSummaryResponseInput::new(
                    step.display_name().to_owned(),
                    step.step_type(),
                    StepSummaryDetails::new(
                        None,
                        step.continues_on_error(),
                        std::time::Duration::ZERO,
                        String::new(),
                        String::new(),
                    ),
                ))
                .with_skip_reason(reason.to_owned())
            })
            .collect();
        JobExecutionResponse::new(
            JobSummaryResponse::new(
                run.job_id().to_string(),
                run.job().name().map(str::to_string),
                steps,
                false,
            )
            .with_skip_reason(reason.to_owned()),
            String::new(),
        )
    }

    fn execute_run(
        &self,
        input: JobExecutionInput<'_>,
    ) -> Result<JobExecutionResponse, Box<dyn Error>> {
        let workflow_name = input
            .workflow()
            .name()
            .or(input.workflow().file())
            .unwrap_or("unnamed");
        self.announce_job_started(workflow_name, input.run());
        let execution = self
            .command_publisher
            .publish(input.execute_job_payload())
            .map_err(|error| Box::new(error) as Box<dyn Error>)?;
        self.announce_job_finished(
            workflow_name,
            input.run(),
            execution.job_summary().success(),
        );
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
