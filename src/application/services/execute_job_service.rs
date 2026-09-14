use std::{collections::HashMap, error::Error, time::Instant};

use crate::application::dtos::requests::BuildJobEnvironmentRequest;
use crate::application::dtos::requests::BuildStepContextRequest;
use crate::application::dtos::requests::ExecuteJobRequest;
use crate::application::dtos::requests::PrefixStepPathRequest;
use crate::application::dtos::requests::PrepareJobContainerRequest;
use crate::application::dtos::requests::ReadStepExportsRequest;
use crate::application::dtos::requests::SummarizeStepRequest;
use crate::application::dtos::responses::JobExecutionResponse;
use crate::application::dtos::responses::JobSummaryResponse;
use crate::application::dtos::responses::{PreparedJobContainerResponse, StepSummaryResponse};
use crate::application::ports::inbound::execute_job_port::ExecuteJobPort;
use crate::application::ports::outbound::build_job_environment_port::BuildJobEnvironmentPort;
use crate::application::ports::outbound::build_step_context_port::BuildStepContextPort;
use crate::application::ports::outbound::command_bus_port::StepCommandBusPort;
use crate::application::ports::outbound::event_bus_port::DomainEventBusPort;
use crate::application::ports::outbound::prefix_step_path_port::PrefixStepPathPort;
use crate::application::ports::outbound::prepare_job_container_port::PrepareJobContainerPort;
use crate::application::ports::outbound::read_step_exports_port::ReadStepExportsPort;
use crate::application::ports::outbound::summarize_step_port::SummarizeStepPort;
use crate::domain::messages::commands::ExecuteStepCommand;
use crate::domain::messages::events::StepStartedPayload;
use crate::domain::messages::events::{DomainEvent, StepFinishedDetails, StepFinishedPayload};

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
    command_bus: Box<StepCommandBusPort>,
    event_bus: Box<DomainEventBusPort>,
}

pub type ExecuteJobStepDependencies = (
    Box<dyn PrefixStepPathPort>,
    Box<dyn BuildStepContextPort>,
    Box<dyn SummarizeStepPort>,
    Box<dyn ReadStepExportsPort>,
);
pub type ExecuteJobMessagingDependencies = (Box<StepCommandBusPort>, Box<DomainEventBusPort>);

pub struct ExecuteJobDependencies {
    job_environment_builder: Box<dyn BuildJobEnvironmentPort>,
    container_preparer: Box<dyn PrepareJobContainerPort>,
    step_path_prefixer: Box<dyn PrefixStepPathPort>,
    step_context_builder: Box<dyn BuildStepContextPort>,
    step_summarizer: Box<dyn SummarizeStepPort>,
    step_exports_reader: Box<dyn ReadStepExportsPort>,
    command_bus: Box<StepCommandBusPort>,
    event_bus: Box<DomainEventBusPort>,
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
        request: ExecuteJobRequest<'_>,
    ) -> Result<JobExecutionResponse, Box<dyn Error>> {
        let mut state = self.prepare_execution(&request)?;
        for step in request.run().job().steps() {
            self.execute_step(&request, step, &mut state);
        }
        Ok(self.build_response(&request, state))
    }
}

impl ExecuteJobService {
    fn prepare_execution(
        &self,
        request: &ExecuteJobRequest<'_>,
    ) -> Result<JobExecutionState, Box<dyn Error>> {
        let step_env = self
            .job_environment_builder
            .execute(BuildJobEnvironmentRequest::new(
                request.workflow(),
                request.run().job().env(),
            ))
            .into_env();
        let prepared = self
            .container_preparer
            .execute(PrepareJobContainerRequest::new(
                request.run().job_id(),
                request.run().job().runs_on(),
                request.repo_path(),
                request.allow_repo_writes(),
            ))?;
        Ok(JobExecutionState::new(step_env, prepared))
    }

    fn execute_step(
        &self,
        request: &ExecuteJobRequest<'_>,
        step: &crate::domain::entities::Step,
        state: &mut JobExecutionState,
    ) {
        state.step_env = self.step_path_prefixer.execute(PrefixStepPathRequest::new(
            &state.step_env,
            &state.extra_path,
        ));
        let started_at = Instant::now();
        let step_context = self
            .step_context_builder
            .execute(BuildStepContextRequest::new(
                request.context(),
                &state.step_env,
            ));
        self.announce_step_started(request, step);
        let outcome = self.command_bus.dispatch(ExecuteStepCommand::new(
            step.clone(),
            state.step_env.clone(),
            step_context,
            state.prepared.container(),
            request.repo_path().to_path_buf(),
        ));
        let summarized = self.step_summarizer.execute(SummarizeStepRequest::new(
            step,
            outcome,
            started_at.elapsed(),
        ));
        state.job_success &= !summarized.fails_job();
        self.announce_step_finished(request, summarized.summary(), !summarized.fails_job());
        state.steps.push(summarized.into_summary());
        let exports = self
            .step_exports_reader
            .execute(ReadStepExportsRequest::new(state.prepared.container()));
        let (path_additions, env) = exports.into_parts();
        state.extra_path.extend(path_additions);
        state.step_env.extend(env);
    }

    fn build_response(
        &self,
        request: &ExecuteJobRequest<'_>,
        state: JobExecutionState,
    ) -> JobExecutionResponse {
        let job_summary = JobSummaryResponse::new(
            request.run().job_id().to_string(),
            request.run().job().name().map(str::to_string),
            state.steps,
            state.job_success,
        );
        JobExecutionResponse::new(job_summary, state.prepared.container_name().to_string())
    }
}

impl ExecuteJobService {
    fn announce_step_started(
        &self,
        request: &ExecuteJobRequest<'_>,
        step: &crate::domain::entities::Step,
    ) {
        self.event_bus
            .publish(DomainEvent::StepStarted(StepStartedPayload::new(
                request.workflow().name().unwrap_or("unnamed").to_string(),
                request.run().job_id().to_string(),
                step.display_name().to_string(),
            )));
    }

    fn announce_step_finished(
        &self,
        request: &ExecuteJobRequest<'_>,
        summary: &StepSummaryResponse,
        step_success: bool,
    ) {
        self.event_bus
            .publish(DomainEvent::StepFinished(StepFinishedPayload::new(
                request.run_id().to_string(),
                StepFinishedDetails::new(
                    request.workflow().name().unwrap_or("unnamed").to_string(),
                    request.run().job_id().to_string(),
                    summary.name().to_string(),
                    step_success,
                    summary.exit_code(),
                )
                .with_stdout(summary.stdout().to_string())
                .with_stderr(summary.stderr().to_string()),
            )));
    }
}
