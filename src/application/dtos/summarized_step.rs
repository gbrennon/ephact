use crate::application::dtos::StepSummary;

/// Summary of one executed step, and whether it fails the job it belongs to.
pub struct SummarizedStep {
    /// Summary reported for the step.
    pub summary: StepSummary,
    /// Whether this step's outcome fails the job.
    pub fails_job: bool,
}

impl SummarizedStep {
    /// Creates a new summarized step.
    pub fn new(summary: StepSummary, fails_job: bool) -> Self {
        Self { summary, fails_job }
    }

    /// Summary reported for the step.
    pub fn summary(&self) -> &StepSummary {
        &self.summary
    }

    /// Consumes the summarized step and returns its summary.
    pub fn into_summary(self) -> StepSummary {
        self.summary
    }

    /// Whether this step's outcome fails the job.
    pub fn fails_job(&self) -> bool {
        self.fails_job
    }
}
