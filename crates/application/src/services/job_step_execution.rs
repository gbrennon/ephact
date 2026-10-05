use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use super::{
    execute_job_dependencies::ExecuteJobDependencies, job_execution_events::JobExecutionEvents,
    job_execution_state::JobExecutionState,
};
use crate::{
    domain::{
        aggregates::Workflow,
        entities::{JobRun, Step},
        messages::commands::ExecuteStepPayload,
        value_objects::EvaluationContext,
    },
    dtos::{
        requests::{
            BuildStepContextRequest, ExecuteJobRequest, ReadStepExportsRequest,
            SummarizeStepRequest,
        },
        responses::{
            PreparedJobContainerResponse, StepSummaryDetails, StepSummaryResponse,
            StepSummaryResponseInput, SummarizedStepResponse,
        },
    },
};

/// Executes and summarizes individual steps within one job execution.
pub struct JobStepExecution<'a> {
    dependencies: &'a ExecuteJobDependencies,
    events: JobExecutionEvents<'a>,
    request: &'a ExecuteJobRequest,
    workflow: &'a Workflow,
    run: &'a JobRun,
    prepared: &'a PreparedJobContainerResponse,
}

impl<'a> JobStepExecution<'a> {
    /// Creates step execution tied to one job's dependencies and context.
    pub fn new(
        dependencies: &'a ExecuteJobDependencies,
        request: &'a ExecuteJobRequest,
        workflow: &'a Workflow,
        run: &'a JobRun,
        prepared: &'a PreparedJobContainerResponse,
    ) -> Self {
        let (_, _, _, _, _, _, _, event_bus, _) = dependencies.as_parts();
        Self {
            dependencies,
            events: JobExecutionEvents::new(event_bus),
            request,
            workflow,
            run,
            prepared,
        }
    }

    /// Executes one step, publishes progress, and records its exports.
    pub fn execute(&self, step: &Step, state: &mut JobExecutionState) {
        let (_, _, step_path_prefixer, step_context_builder, _, step_exports_reader, _, _, _) =
            self.dependencies.as_parts();
        let (step_context, step_env) = state.prepare_step(self.request, step_path_prefixer);
        let started_at = Instant::now();
        let step_context = step_context_builder
            .build(BuildStepContextRequest::new(step_context, step_env.clone()));
        self.events
            .publish_step_started(self.workflow, self.run, step);
        let summarized = self.summarize_step(step, step_context, step_env, started_at);
        self.events
            .publish_step_finished(self.request, self.workflow, self.run, &summarized);
        let exports =
            step_exports_reader.read(ReadStepExportsRequest::new(), self.prepared.container());
        state.complete_step(
            step,
            summarized,
            Some(exports),
            self.run.job().continues_after_failure(),
        );
    }

    /// Records a step skipped after an earlier step failed.
    pub fn skip(&self, step: &Step, state: &mut JobExecutionState, continues_after_failure: bool) {
        let skipped = SummarizedStepResponse::new(
            StepSummaryResponse::new(StepSummaryResponseInput::new(
                step.display_name().to_string(),
                step.step_type(),
                StepSummaryDetails::new(
                    None,
                    step.continues_on_error(),
                    Duration::ZERO,
                    "",
                    "previous step failed",
                ),
            ))
            .with_skip_reason("previous step failed"),
            false,
        );
        state.complete_step(step, skipped, None, continues_after_failure);
    }

    fn summarize_step(
        &self,
        step: &Step,
        step_context: EvaluationContext,
        step_env: HashMap<String, String>,
        started_at: Instant,
    ) -> SummarizedStepResponse {
        let (_, _, _, _, step_summarizer, _, command_bus, _, network_command_classifier) =
            self.dependencies.as_parts();
        let reason = step
            .network_policy_violation(network_command_classifier)
            .or_else(|| {
                (!self.request.allow_network())
                    .then(|| step.network_access_reason(network_command_classifier))
                    .flatten()
            });
        if let Some(reason) = reason {
            return SummarizedStepResponse::new(
                StepSummaryResponse::new(StepSummaryResponseInput::new(
                    step.display_name().to_string(),
                    step.step_type(),
                    StepSummaryDetails::new(
                        None,
                        step.continues_on_error(),
                        started_at.elapsed(),
                        "",
                        reason,
                    ),
                ))
                .with_skip_reason(reason),
                false,
            );
        }
        let outcome = command_bus.publish(
            ExecuteStepPayload::new(
                step.clone(),
                step_env,
                step_context,
                self.request.repo_path().to_path_buf(),
            ),
            self.prepared.container_handle(),
        );
        step_summarizer.summarize(SummarizeStepRequest::new(
            step,
            outcome,
            started_at.elapsed(),
        ))
    }
}
