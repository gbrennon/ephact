use super::step_summary_response::StepSummaryResponse;

/// Summary of one executed step, and whether it fails the job it belongs to.
pub struct SummarizedStepResponse {
    summary: StepSummaryResponse,
    fails_job: bool,
}

impl SummarizedStepResponse {
    pub fn new(summary: StepSummaryResponse, fails_job: bool) -> Self {
        Self { summary, fails_job }
    }
    pub fn summary(&self) -> &StepSummaryResponse {
        &self.summary
    }
    pub fn into_summary(self) -> StepSummaryResponse {
        self.summary
    }
    pub fn fails_job(&self) -> bool {
        self.fails_job
    }
}
