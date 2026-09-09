use std::{error::Error, sync::Arc, time::Instant};

use crate::application::commands::ExecuteStepCommand;
use crate::application::dtos::BuildJobEnvironmentRequest;
use crate::application::dtos::BuildStepContextRequest;
use crate::application::dtos::ExecuteJobRequest;
use crate::application::dtos::JobExecution;
use crate::application::dtos::JobSummary;
use crate::application::dtos::PrefixStepPathRequest;
use crate::application::dtos::PrepareJobContainerRequest;
use crate::application::dtos::ReadStepExportsRequest;
use crate::application::dtos::StepSummary;
use crate::application::dtos::SummarizeStepRequest;
use crate::application::ports::inbound::execute_job_port::ExecuteJobPort;
use crate::application::ports::outbound::build_job_environment_port::BuildJobEnvironmentPort;
use crate::application::ports::outbound::build_step_context_port::BuildStepContextPort;
use crate::application::ports::outbound::command_bus_port::CommandBusPort;
use crate::application::ports::outbound::event_bus_port::EventBusPort;
use crate::application::ports::outbound::prefix_step_path_port::PrefixStepPathPort;
use crate::application::ports::outbound::prepare_job_container_port::PrepareJobContainerPort;
use crate::application::ports::outbound::read_step_exports_port::ReadStepExportsPort;
use crate::application::ports::outbound::summarize_step_port::SummarizeStepPort;
use crate::domain::events::{DomainEvent, StepFinishedPayload, StepStartedPayload};

/// Application service coordinating the execution of one job.
///
/// Builds the job environment and container through outbound ports, then
/// publishes one [`ExecuteStepCommand`] per step: the step command handler
/// runs each step, so this service never depends on the step entrypoint.
/// Progress facts for every step are announced as domain events on the
/// outbound [`EventBusPort`].
pub struct ExecuteJobService {
    job_environment_builder: Box<dyn BuildJobEnvironmentPort>,
    container_preparer: Box<dyn PrepareJobContainerPort>,
    step_path_prefixer: Box<dyn PrefixStepPathPort>,
    step_context_builder: Box<dyn BuildStepContextPort>,
    step_summarizer: Box<dyn SummarizeStepPort>,
    step_exports_reader: Box<dyn ReadStepExportsPort>,
    command_bus: Arc<dyn CommandBusPort>,
    event_bus: Arc<dyn EventBusPort>,
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
        command_bus: Arc<dyn CommandBusPort>,
        event_bus: Arc<dyn EventBusPort>,
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
    fn execute(&self, request: ExecuteJobRequest<'_>) -> Result<JobExecution, Box<dyn Error>> {
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
        let mut steps: Vec<StepSummary> = Vec::new();

        for step in request.run().job().steps() {
            step_env = self
                .step_path_prefixer
                .execute(PrefixStepPathRequest::new(&step_env, &extra_path));

            let started_at = Instant::now();
            let step_context = self
                .step_context_builder
                .execute(BuildStepContextRequest::new(request.context(), &step_env));

            self.announce_step_started(&request, step);
            let outcome = self.command_bus.dispatch_step(ExecuteStepCommand::new(
                step.clone(),
                step_env.clone(),
                step_context,
                prepared.container().clone(),
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
                .execute(ReadStepExportsRequest::new(prepared.container().as_ref()));
            let (path_additions, env) = exports.into_parts();
            extra_path.extend(path_additions);
            step_env.extend(env);
        }

        let job_summary = JobSummary::new(
            request.run().job_id().to_string(),
            request.run().job().name().map(str::to_string),
            steps,
            job_success,
        );
        Ok(JobExecution::new(
            job_summary,
            prepared.container_name().to_string(),
        ))
    }
}

impl ExecuteJobService {
    fn announce_step_started(
        &self,
        request: &ExecuteJobRequest<'_>,
        step: &crate::domain::workflow::Step,
    ) {
        self.event_bus
            .publish(DomainEvent::StepStarted(StepStartedPayload::new(
                request.workflow().name().unwrap_or("unnamed").to_string(),
                request.run().job_id().to_string(),
                step.name().unwrap_or("unnamed step").to_string(),
            )));
    }

    fn announce_step_finished(
        &self,
        request: &ExecuteJobRequest<'_>,
        summary: &StepSummary,
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
