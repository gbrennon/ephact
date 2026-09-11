use std::{error::Error, time::Instant};

use crate::application::dtos::requests::BuildJobEnvironmentRequest;
use crate::application::dtos::requests::BuildStepContextRequest;
use crate::application::dtos::requests::ExecuteJobRequest;
use crate::application::dtos::requests::PrefixStepPathRequest;
use crate::application::dtos::requests::PrepareJobContainerRequest;
use crate::application::dtos::requests::ReadStepExportsRequest;
use crate::application::dtos::requests::SummarizeStepRequest;
use crate::application::dtos::responses::JobExecutionResponse;
use crate::application::dtos::responses::JobSummaryResponse;
use crate::application::dtos::responses::StepSummaryResponse;
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
use crate::domain::messages::events::DomainEvent;
use crate::domain::messages::events::StepFinishedPayload;
use crate::domain::messages::events::StepStartedPayload;

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

impl ExecuteJobService {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        job_environment_builder: Box<dyn BuildJobEnvironmentPort>,
        container_preparer: Box<dyn PrepareJobContainerPort>,
        step_path_prefixer: Box<dyn PrefixStepPathPort>,
        step_context_builder: Box<dyn BuildStepContextPort>,
        step_summarizer: Box<dyn SummarizeStepPort>,
        step_exports_reader: Box<dyn ReadStepExportsPort>,
        command_bus: Box<StepCommandBusPort>,
        event_bus: Box<DomainEventBusPort>,
    ) -> Self {
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

impl ExecuteJobPort for ExecuteJobService {
    fn execute(
        &self,
        request: ExecuteJobRequest<'_>,
    ) -> Result<JobExecutionResponse, Box<dyn Error>> {
        let mut step_env = self
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
            ))?;

        let mut extra_path: Vec<String> = Vec::new();
        let mut job_success = true;
        let mut steps: Vec<StepSummaryResponse> = Vec::new();

        for step in request.run().job().steps() {
            step_env = self
                .step_path_prefixer
                .execute(PrefixStepPathRequest::new(&step_env, &extra_path));

            let started_at = Instant::now();
            let step_context = self
                .step_context_builder
                .execute(BuildStepContextRequest::new(request.context(), &step_env));

            self.announce_step_started(&request, step);
            let outcome = self.command_bus.dispatch(ExecuteStepCommand::new(
                step.clone(),
                step_env.clone(),
                step_context,
                prepared.container(),
                request.repo_path().to_path_buf(),
            ));

            let summarized = self.step_summarizer.execute(SummarizeStepRequest::new(
                step,
                outcome,
                started_at.elapsed(),
            ));
            job_success &= !summarized.fails_job();
            self.announce_step_finished(&request, summarized.summary(), !summarized.fails_job());
            steps.push(summarized.summary().clone());

            let exports = self
                .step_exports_reader
                .execute(ReadStepExportsRequest::new(prepared.container()));
            let (path_additions, env) = exports.into_parts();
            extra_path.extend(path_additions);
            step_env.extend(env);
        }

        let job_summary = JobSummaryResponse::new(
            request.run().job_id().to_string(),
            request.run().job().name().map(str::to_string),
            steps,
            job_success,
        );
        Ok(JobExecutionResponse::new(
            job_summary,
            prepared.container_name().to_string(),
        ))
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
                request.workflow().name().unwrap_or("unnamed").to_string(),
                request.run().job_id().to_string(),
                summary.name().to_string(),
                step_success,
                summary.exit_code(),
                summary.stdout().to_string(),
                summary.stderr().to_string(),
            )));
    }
}
