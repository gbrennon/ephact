use std::{collections::HashMap, error::Error, time::Instant};

use crate::{
    application::{
        dtos::{
            requests::{
                BuildJobEnvironmentRequest, BuildStepContextRequest, ExecuteJobRequest,
                PrefixStepPathRequest, PrepareJobContainerRequest, ReadStepExportsRequest,
                SummarizeStepRequest,
            },
            responses::{
                JobExecutionResponse, JobSummaryResponse, PreparedJobContainerResponse,
                StepSummaryDetails, StepSummaryResponse, StepSummaryResponseInput,
            },
        },
        errors::ExecuteJobError,
        ports::{
            inbound::execute_job_port::ExecuteJobPort,
            outbound::{
                build_job_environment_port::BuildJobEnvironmentPort,
                build_step_context_port::BuildStepContextPort,
                domain_event_bus_port::DomainEventBusPort,
                prefix_step_path_port::PrefixStepPathPort,
                prepare_job_container_port::PrepareJobContainerPort,
                read_step_exports_port::ReadStepExportsPort,
                step_command_bus_port::StepCommandBusPort, summarize_step_port::SummarizeStepPort,
            },
        },
    },
    domain::messages::{
        commands::ExecuteStepCommand,
        events::{
            ContainerStartedPayload, DomainEvent, StepFinishedDetails, StepFinishedPayload,
            StepStartedPayload,
        },
    },
};

/// Application service coordinating the execution of one job.
///
/// Builds the job environment and container through outbound ports, then
/// publishes one [`ExecuteStepCommand`] per step: the step command handler
/// runs each step, so this service never depends on the step entrypoint.
/// Progress facts for every step are announced as domain events on the
/// outbound [`DomainEventBusPort`].
pub struct ExecuteJobService {
    job_environment_builder: Box<dyn BuildJobEnvironmentPort>,
    container_preparer: Box<dyn PrepareJobContainerPort>,
    step_path_prefixer: Box<dyn PrefixStepPathPort>,
    step_context_builder: Box<dyn BuildStepContextPort>,
    step_summarizer: Box<dyn SummarizeStepPort>,
    step_exports_reader: Box<dyn ReadStepExportsPort>,
    command_bus: Box<dyn StepCommandBusPort>,
    event_bus: Box<dyn DomainEventBusPort>,
}

pub type ExecuteJobStepDependencies = (
    Box<dyn PrefixStepPathPort>,
    Box<dyn BuildStepContextPort>,
    Box<dyn SummarizeStepPort>,
    Box<dyn ReadStepExportsPort>,
);
pub type ExecuteJobMessagingDependencies =
    (Box<dyn StepCommandBusPort>, Box<dyn DomainEventBusPort>);

pub struct ExecuteJobDependencies {
    job_environment_builder: Box<dyn BuildJobEnvironmentPort>,
    container_preparer: Box<dyn PrepareJobContainerPort>,
    step_path_prefixer: Box<dyn PrefixStepPathPort>,
    step_context_builder: Box<dyn BuildStepContextPort>,
    step_summarizer: Box<dyn SummarizeStepPort>,
    step_exports_reader: Box<dyn ReadStepExportsPort>,
    command_bus: Box<dyn StepCommandBusPort>,
    event_bus: Box<dyn DomainEventBusPort>,
}

impl ExecuteJobDependencies {
    pub fn new(
        job_environment_builder: Box<dyn BuildJobEnvironmentPort>,
        container_preparer: Box<dyn PrepareJobContainerPort>,
        step_dependencies: ExecuteJobStepDependencies,
        messaging_dependencies: ExecuteJobMessagingDependencies,
    ) -> Self {
        let (step_path_prefixer, step_context_builder, step_summarizer, step_exports_reader) =
            step_dependencies;
        let (command_bus, event_bus) = messaging_dependencies;
        Self {
            job_environment_builder,
            container_preparer,
            step_path_prefixer,
            step_context_builder,
            step_summarizer,
            step_exports_reader,
            command_bus,
            event_bus,
        }
    }
}

struct JobExecutionState {
    step_env: HashMap<String, String>,
    extra_path: Vec<String>,
    prepared: PreparedJobContainerResponse,
    steps: Vec<StepSummaryResponse>,
    job_success: bool,
}

impl JobExecutionState {
    fn new(step_env: HashMap<String, String>, prepared: PreparedJobContainerResponse) -> Self {
        Self {
            step_env,
            extra_path: Vec::new(),
            prepared,
            steps: Vec::new(),
            job_success: true,
        }
    }
}

impl ExecuteJobService {
    pub fn new(dependencies: ExecuteJobDependencies) -> Self {
        Self {
            job_environment_builder: dependencies.job_environment_builder,
            container_preparer: dependencies.container_preparer,
            step_path_prefixer: dependencies.step_path_prefixer,
            step_context_builder: dependencies.step_context_builder,
            step_summarizer: dependencies.step_summarizer,
            step_exports_reader: dependencies.step_exports_reader,
            command_bus: dependencies.command_bus,
            event_bus: dependencies.event_bus,
        }
    }
}

impl ExecuteJobPort for ExecuteJobService {
    fn execute(
        &self,
        request: ExecuteJobRequest,
        run: &crate::domain::entities::JobRun,
        workflow: &crate::domain::aggregates::Workflow,
    ) -> Result<JobExecutionResponse, ExecuteJobError> {
        let mut state = self
            .prepare_execution(&request, run, workflow)
            .map_err(|error| ExecuteJobError::Preparation(error.to_string()))?;
        self.announce_container_started(&request, &state);
        for step in run.job().steps() {
            self.execute_step(&request, workflow, run, step, &mut state);
        }
        Ok(self.build_response(&request, run, state))
    }
}

impl ExecuteJobService {
    fn prepare_execution(
        &self,
        request: &ExecuteJobRequest,
        run: &crate::domain::entities::JobRun,
        workflow: &crate::domain::aggregates::Workflow,
    ) -> Result<JobExecutionState, Box<dyn Error>> {
        let step_env = self
            .job_environment_builder
            .execute(BuildJobEnvironmentRequest::new(
                workflow.clone(),
                run.job().env().clone(),
            ))
            .into_env();
        let prepared = self
            .container_preparer
            .execute(PrepareJobContainerRequest::new(
                run.job_id().to_string(),
                run.job().runs_on().map(str::to_string),
                request.repo_path().to_path_buf(),
                request.allow_repo_writes(),
            ))?;
        Ok(JobExecutionState::new(step_env, prepared))
    }

    fn announce_container_started(&self, request: &ExecuteJobRequest, state: &JobExecutionState) {
        self.event_bus
            .publish(DomainEvent::ContainerStarted(ContainerStartedPayload::new(
                request.run_id().to_string(),
                state.prepared.container_name().to_string(),
            )));
    }

    fn execute_step(
        &self,
        request: &ExecuteJobRequest,
        workflow: &crate::domain::aggregates::Workflow,
        run: &crate::domain::entities::JobRun,
        step: &crate::domain::entities::Step,
        state: &mut JobExecutionState,
    ) {
        state.step_env = self.step_path_prefixer.execute(PrefixStepPathRequest::new(
            state.step_env.clone(),
            state.extra_path.clone(),
        ));
        let started_at = Instant::now();
        let step_context = self
            .step_context_builder
            .execute(BuildStepContextRequest::new(
                request.context().to_vec(),
                state.step_env.clone(),
            ));
        self.announce_step_started(request, workflow, run, step);
        let summarized = self.summarize_step(request, step, state, step_context, started_at);
        state.job_success &= !summarized.fails_job();
        self.announce_step_finished(
            request,
            workflow,
            run,
            summarized.summary(),
            !summarized.fails_job(),
        );
        state.steps.push(summarized.into_summary());
        let exports = self
            .step_exports_reader
            .execute(ReadStepExportsRequest::new(), state.prepared.container());
        let (path_additions, env) = exports.into_parts();
        state.extra_path.extend(path_additions);
        state.step_env.extend(env);
    }

    fn summarize_step(
        &self,
        request: &ExecuteJobRequest,
        step: &crate::domain::entities::Step,
        state: &JobExecutionState,
        step_context: crate::domain::value_objects::EvaluationContext,
        started_at: Instant,
    ) -> crate::application::dtos::responses::SummarizedStepResponse {
        if !request.allow_network()
            && let Some(reason) = step.network_access_reason()
        {
            return self.skipped_step(step, started_at.elapsed(), reason);
        }
        let outcome = self.command_bus.dispatch(ExecuteStepCommand::new(
            step.clone(),
            state.step_env.clone(),
            step_context,
            state.prepared.container_handle(),
            request.repo_path().to_path_buf(),
        ));
        self.step_summarizer.execute(SummarizeStepRequest::new(
            step,
            outcome,
            started_at.elapsed(),
        ))
    }

    fn skipped_step(
        &self,
        step: &crate::domain::entities::Step,
        duration: std::time::Duration,
        reason: &str,
    ) -> crate::application::dtos::responses::SummarizedStepResponse {
        let summary = StepSummaryResponse::new(StepSummaryResponseInput::new(
            step.display_name().to_string(),
            step.step_type(),
            StepSummaryDetails::new(None, step.continues_on_error(), duration, "", reason),
        ))
        .with_skip_reason(reason);
        crate::application::dtos::responses::SummarizedStepResponse::new(summary, false)
    }

    fn build_response(
        &self,
        _request: &ExecuteJobRequest,
        run: &crate::domain::entities::JobRun,
        state: JobExecutionState,
    ) -> JobExecutionResponse {
        let job_summary = JobSummaryResponse::new(
            run.job_id().to_string(),
            run.job().name().map(str::to_string),
            state.steps,
            state.job_success,
        );
        JobExecutionResponse::new(job_summary, state.prepared.container_name().to_string())
    }
}

impl ExecuteJobService {
    fn announce_step_started(
        &self,
        _request: &ExecuteJobRequest,
        workflow: &crate::domain::aggregates::Workflow,
        run: &crate::domain::entities::JobRun,
        step: &crate::domain::entities::Step,
    ) {
        self.event_bus
            .publish(DomainEvent::StepStarted(StepStartedPayload::new(
                workflow.name().unwrap_or("unnamed").to_string(),
                run.job_id().to_string(),
                step.display_name().to_string(),
            )));
    }

    fn announce_step_finished(
        &self,
        request: &ExecuteJobRequest,
        workflow: &crate::domain::aggregates::Workflow,
        run: &crate::domain::entities::JobRun,
        summary: &StepSummaryResponse,
        step_success: bool,
    ) {
        self.event_bus
            .publish(DomainEvent::StepFinished(StepFinishedPayload::new(
                request.run_id().to_string(),
                StepFinishedDetails::new(
                    workflow.name().unwrap_or("unnamed").to_string(),
                    run.job_id().to_string(),
                    summary.name().to_string(),
                    step_success,
                    summary.exit_code(),
                )
                .with_stdout(summary.stdout().to_string())
                .with_stderr(summary.stderr().to_string()),
            )));
    }
}
