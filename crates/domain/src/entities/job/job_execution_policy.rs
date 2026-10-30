use crate::value_objects::{ConcurrencyGroup, JobStrategy, TokenPermissions};

#[derive(Debug, Clone, PartialEq, Default)]
pub(super) struct JobExecutionPolicy {
    /// An expression that determines whether the job runs.
    r#if: Option<String>,

    /// A matrix strategy to generate multiple job runs.
    strategy: Option<JobStrategy>,

    /// Maximum number of minutes to let the job run.
    timeout_minutes: Option<f64>,

    /// Whether to continue the workflow even if this job fails.
    continue_on_error: Option<String>,

    /// TokenPermissions override for this job.
    permissions: Option<TokenPermissions>,

    /// ConcurrencyGroup override for this job.
    concurrency: Option<ConcurrencyGroup>,
}

impl JobExecutionPolicy {
    pub(super) fn with_if_condition(mut self, condition: Option<String>) -> Self {
        self.r#if = condition;
        self
    }

    pub(super) fn with_strategy(mut self, strategy: Option<JobStrategy>) -> Self {
        self.strategy = strategy;
        self
    }

    pub(super) fn with_timeout_minutes(mut self, timeout_minutes: Option<f64>) -> Self {
        self.timeout_minutes = timeout_minutes;
        self
    }

    pub(super) fn with_continue_on_error(mut self, value: Option<String>) -> Self {
        self.continue_on_error = value;
        self
    }

    pub(super) fn with_permissions(mut self, permissions: Option<TokenPermissions>) -> Self {
        self.permissions = permissions;
        self
    }

    pub(super) fn with_concurrency(mut self, concurrency: Option<ConcurrencyGroup>) -> Self {
        self.concurrency = concurrency;
        self
    }

    pub(super) fn if_condition(&self) -> Option<&str> {
        self.r#if.as_deref()
    }

    pub(super) fn strategy(&self) -> Option<&JobStrategy> {
        self.strategy.as_ref()
    }

    pub(super) fn timeout_minutes(&self) -> Option<f64> {
        self.timeout_minutes
    }

    pub(super) fn continue_on_error(&self) -> Option<&str> {
        self.continue_on_error.as_deref()
    }

    pub(super) fn permissions(&self) -> Option<&TokenPermissions> {
        self.permissions.as_ref()
    }

    pub(super) fn concurrency(&self) -> Option<&ConcurrencyGroup> {
        self.concurrency.as_ref()
    }

    pub(super) fn continues_after_failure(&self) -> bool {
        self.continue_on_error()
            .is_some_and(|value| value.eq_ignore_ascii_case("true"))
    }
}
