use crate::value_objects::{ConcurrencyGroup, JobStrategy, TokenPermissions};

#[derive(Debug, Clone, PartialEq, Default)]
pub(super) struct JobExecutionPolicy {
    /// An expression that determines whether the job runs.
    pub(super) r#if: Option<String>,

    /// A matrix strategy to generate multiple job runs.
    pub(super) strategy: Option<JobStrategy>,

    /// Maximum number of minutes to let the job run.
    pub(super) timeout_minutes: Option<f64>,

    /// Whether to continue the workflow even if this job fails.
    pub(super) continue_on_error: Option<String>,

    /// TokenPermissions override for this job.
    pub(super) permissions: Option<TokenPermissions>,

    /// ConcurrencyGroup override for this job.
    pub(super) concurrency: Option<ConcurrencyGroup>,
}
