use std::collections::{BTreeMap, HashMap};

use crate::{
    domain::{
        entities::{JobRun, Step},
        value_objects::{ContextValue, EvaluationContext},
    },
    dtos::{
        requests::{ExecuteJobRequest, PrefixStepPathRequest},
        responses::{
            JobExecutionResponse, JobSummaryResponse, StepExportsResponse, StepSummaryResponse,
            SummarizedStepResponse,
        },
    },
    ports::outbound::step_path_prefixer_port::StepPathPrefixerPort,
};

/// Mutable state accumulated while one job run executes.
pub struct JobExecutionState {
    step_env: HashMap<String, String>,
    extra_path: Vec<String>,
    step_outputs: BTreeMap<String, ContextValue>,
    steps: Vec<StepSummaryResponse>,
    job_success: bool,
}

impl JobExecutionState {
    /// Creates initial state for a job execution.
    pub fn new(step_env: HashMap<String, String>) -> Self {
        Self {
            step_env,
            extra_path: Vec::new(),
            step_outputs: BTreeMap::new(),
            steps: Vec::new(),
            job_success: true,
        }
    }

    /// Returns whether the next step should be skipped for this job.
    pub fn should_skip_step(&self, continues_after_failure: bool) -> bool {
        !self.job_success && !continues_after_failure
    }

    /// Prepares the environment and evaluation context for the next step.
    pub fn prepare_step(
        &mut self,
        request: &ExecuteJobRequest,
        path_prefixer: &dyn StepPathPrefixerPort,
    ) -> (EvaluationContext, HashMap<String, String>) {
        self.step_env = path_prefixer.prefix(PrefixStepPathRequest::new(
            self.step_env.clone(),
            self.extra_path.clone(),
        ));
        let step_context = request
            .context()
            .clone()
            .with_root("steps", ContextValue::Mapping(self.step_outputs.clone()));
        (step_context, self.step_env.clone())
    }

    /// Records a step result and merges any exports made by that step.
    pub fn complete_step(
        &mut self,
        step: &Step,
        summarized: SummarizedStepResponse,
        exports: Option<StepExportsResponse>,
        continues_after_failure: bool,
    ) {
        let fails_job = summarized.fails_job();
        self.job_success &= !fails_job || continues_after_failure;
        self.steps.push(summarized.into_summary());
        if let Some(exports) = exports {
            let (path_additions, env, outputs) = exports.into_parts();
            self.extra_path.extend(path_additions);
            self.step_env.extend(env);
            if let Some(step_id) = step.id()
                && !outputs.is_empty()
            {
                let output_values = ContextValue::mapping(
                    outputs
                        .into_iter()
                        .map(|(name, value)| (name, ContextValue::text(value))),
                );
                self.step_outputs.insert(
                    step_id.to_owned(),
                    ContextValue::mapping([("outputs".to_owned(), output_values)]),
                );
            }
        }
    }

    /// Consumes state and builds the completed job response.
    pub fn into_response(self, run: &JobRun, container_name: String) -> JobExecutionResponse {
        let job_summary = JobSummaryResponse::new(
            run.job_id().to_string(),
            run.job().name().map(str::to_string),
            self.steps,
            self.job_success,
        );
        JobExecutionResponse::new(job_summary, container_name)
    }
}
