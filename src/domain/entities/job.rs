use std::collections::HashMap;

use crate::domain::{
    entities::Step,
    value_objects::{
        ConcurrencyGroup, ContainerSpecification, ContextValue, JobStrategy, TokenPermissions,
    },
};

/// A job in a workflow.
///
/// Jobs run in parallel by default but can be sequenced with `needs`.
/// Each job runs on a fresh virtual environment specified by `runs_on`.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
///
/// use ephact::domain::entities::Job;
///
/// let job = Job::new(
///     None,
///     Some("ubuntu-latest".to_owned()),
///     Vec::new(),
///     Vec::new(),
///     None,
///     None,
///     HashMap::new(),
///     None,
///     HashMap::new(),
///     HashMap::new(),
///     None,
///     None,
///     None,
///     None,
///     None,
///     None,
/// );
///
/// assert_eq!(job.runs_on(), Some("ubuntu-latest"));
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Job {
    /// The display name of the job.
    name: Option<String>,

    /// The type of machine to run the job on (e.g. `ubuntu-latest`).
    runs_on: Option<String>,

    /// The sequence of steps to execute.
    steps: Vec<Step>,

    /// Jobs that must complete successfully before this job runs.
    needs: Vec<String>,

    /// An expression that determines whether the job runs.
    r#if: Option<String>,

    /// A matrix strategy to generate multiple job runs.
    strategy: Option<JobStrategy>,

    /// Environment variables scoped to this job.
    env: HashMap<String, String>,

    /// Container to run the job inside.
    container: Option<ContainerSpecification>,

    /// Service containers to run alongside the job.
    services: HashMap<String, ContainerSpecification>,

    /// Outputs produced by this job (for dependent jobs).
    outputs: HashMap<String, String>,

    /// Input parameters passed via `workflow_call`.
    with: Option<ContextValue>,

    /// Secrets available to this job.
    secrets: Option<ContextValue>,

    /// Maximum number of minutes to let the job run.
    timeout_minutes: Option<f64>,

    /// Whether to continue the workflow even if this job fails.
    continue_on_error: Option<String>,

    /// TokenPermissions override for this job.
    permissions: Option<TokenPermissions>,

    /// ConcurrencyGroup override for this job.
    concurrency: Option<ConcurrencyGroup>,
}

impl Job {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: Option<String>,
        runs_on: Option<String>,
        steps: Vec<Step>,
        needs: Vec<String>,
        r#if: Option<String>,
        strategy: Option<JobStrategy>,
        env: HashMap<String, String>,
        container: Option<ContainerSpecification>,
        services: HashMap<String, ContainerSpecification>,
        outputs: HashMap<String, String>,
        with: Option<ContextValue>,
        secrets: Option<ContextValue>,
        timeout_minutes: Option<f64>,
        continue_on_error: Option<String>,
        permissions: Option<TokenPermissions>,
        concurrency: Option<ConcurrencyGroup>,
    ) -> Self {
        Self {
            name,
            runs_on,
            steps,
            needs,
            r#if,
            strategy,
            env,
            container,
            services,
            outputs,
            with,
            secrets,
            timeout_minutes,
            continue_on_error,
            permissions,
            concurrency,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn runs_on(&self) -> Option<&str> {
        self.runs_on.as_deref()
    }

    pub fn steps(&self) -> &[Step] {
        &self.steps
    }

    pub fn needs(&self) -> &[String] {
        &self.needs
    }

    pub fn r#if(&self) -> Option<&str> {
        self.r#if.as_deref()
    }

    pub fn if_condition(&self) -> Option<&str> {
        self.r#if.as_deref()
    }

    pub fn strategy(&self) -> Option<&JobStrategy> {
        self.strategy.as_ref()
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn container(&self) -> Option<&ContainerSpecification> {
        self.container.as_ref()
    }

    pub fn services(&self) -> &HashMap<String, ContainerSpecification> {
        &self.services
    }

    pub fn outputs(&self) -> &HashMap<String, String> {
        &self.outputs
    }

    pub fn with(&self) -> Option<&ContextValue> {
        self.with.as_ref()
    }

    pub fn secrets(&self) -> Option<&ContextValue> {
        self.secrets.as_ref()
    }

    pub fn timeout_minutes(&self) -> Option<f64> {
        self.timeout_minutes
    }

    pub fn continue_on_error(&self) -> Option<&str> {
        self.continue_on_error.as_deref()
    }

    pub fn permissions(&self) -> Option<&TokenPermissions> {
        self.permissions.as_ref()
    }

    pub fn concurrency(&self) -> Option<&ConcurrencyGroup> {
        self.concurrency.as_ref()
    }
    /// Returns whether this job depends directly on `job_id`.
    pub fn depends_on(&self, job_id: &str) -> bool {
        self.needs.iter().any(|dependency| dependency == job_id)
    }

    /// Returns whether a failed step should not stop this job.
    pub fn continues_after_failure(&self) -> bool {
        self.continue_on_error
            .as_deref()
            .is_some_and(|value| value.eq_ignore_ascii_case("true"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_and_accessors_preserve_fields() {
        let job = Job::new(
            Some("Build".into()),
            Some("ubuntu".into()),
            Vec::new(),
            vec!["setup".into()],
            Some("always()".into()),
            None,
            HashMap::from([("KEY".into(), "value".into())]),
            None,
            HashMap::new(),
            HashMap::from([("output".into(), "value".into())]),
            Some(ContextValue::mapping([(
                "token".to_owned(),
                ContextValue::text("value"),
            )])),
            Some(ContextValue::empty_mapping()),
            Some(10.0),
            Some("true".into()),
            None,
            None,
        );

        assert_eq!(job.name(), Some("Build"));
        assert_eq!(job.runs_on(), Some("ubuntu"));
        assert!(job.steps().is_empty());
        assert_eq!(job.needs(), &["setup".to_string()]);
        assert_eq!(job.r#if(), Some("always()"));
        assert_eq!(job.if_condition(), Some("always()"));
        assert_eq!(job.env()["KEY"], "value");
        assert!(job.services().is_empty());
        assert_eq!(job.outputs()["output"], "value");
        assert_eq!(
            job.with().and_then(|with| with.property("token")),
            Some(&ContextValue::text("value"))
        );
        assert_eq!(job.secrets(), Some(&ContextValue::empty_mapping()));
        assert_eq!(job.timeout_minutes(), Some(10.0));
        assert_eq!(job.continue_on_error(), Some("true"));
        assert!(job.strategy().is_none());
        assert!(job.container().is_none());
        assert!(job.permissions().is_none());
        assert!(job.concurrency().is_none());
    }
    #[test]
    fn exposes_dependency_and_failure_policy_behavior() {
        let job = Job::new(
            None,
            None,
            Vec::new(),
            vec!["setup".into()],
            None,
            None,
            HashMap::new(),
            None,
            HashMap::new(),
            HashMap::new(),
            None,
            None,
            None,
            Some("true".into()),
            None,
            None,
        );

        assert!(job.depends_on("setup"));
        assert!(!job.depends_on("build"));
        assert!(job.continues_after_failure());
    }
}
