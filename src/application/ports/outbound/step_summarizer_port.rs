use crate::application::dtos::{requests::SummarizeStepRequest, responses::SummarizedStepResponse};

/// Converts a step outcome into a run-summary entry.
pub trait StepSummarizerPort: Send + Sync {
    /// Summarises the step and reports whether it fails the job.
    fn summarize(&self, request: SummarizeStepRequest<'_>) -> SummarizedStepResponse;
}
