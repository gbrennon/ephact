use crate::application::dtos::responses::StepSummaryResponse;

/// Summary of one executed step, and whether it fails the job it belongs to.
pub struct SummarizedStepResponse {
    /// Summary reported for the step.
    summary: StepSummaryResponse,
    /// Whether this step's outcome fails the job.
    fails_job: bool,
}

impl SummarizedStepResponse {
    /// Creates a new summarized step.
    pub fn new(summary: StepSummaryResponse, fails_job: bool) -> Self {
        Self { summary, fails_job }
    }

    /// Summary reported for the step.
    pub fn summary(&self) -> &StepSummaryResponse {
        &self.summary
    }

    /// Consumes the summarized step and returns its summary.
    pub fn into_summary(self) -> StepSummaryResponse {
        self.summary
    }

    /// Whether this step's outcome fails the job.
    pub fn fails_job(&self) -> bool {
        self.fails_job
    }
}
