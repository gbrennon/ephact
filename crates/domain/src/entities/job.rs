use std::collections::HashMap;

use crate::{
    entities::Step,
    value_objects::{
        ConcurrencyGroup, ContainerSpecification, ContextValue, JobStrategy, TokenPermissions,
    },
};

/// A workflow job containing steps, declared dependencies, environment, and
/// optional container and execution settings.
///
/// `needs` contributes dependency ordering during workflow planning.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
///
/// use ephact_domain::entities::Job;
///
/// let job = Job::new(
///     None,
///     Some("ubuntu-latest".to_owned()),
///     Vec::new(),
///     Vec::new(),
/// );
///
/// assert_eq!(job.runs_on(), Some("ubuntu-latest"));
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Job {
    identity: JobIdentity,
    policy: JobExecutionPolicy,
    runtime: JobRuntime,
    contract: JobContract,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct JobIdentity {
    /// The display name of the job.
    name: Option<String>,

    /// Jobs that must complete successfully before this job runs.
    needs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct JobExecutionPolicy {
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

#[derive(Debug, Clone, PartialEq, Default)]
struct JobRuntime {
    /// The type of machine to run the job on (e.g. `ubuntu-latest`).
    runs_on: Option<String>,

    /// The sequence of steps to execute.
    steps: Vec<Step>,

    /// Environment variables scoped to this job.
    env: HashMap<String, String>,

    /// Container to run the job inside.
    container: Option<ContainerSpecification>,

    /// Service containers to run alongside the job.
    services: HashMap<String, ContainerSpecification>,
}

#[derive(Debug, Clone, PartialEq, Default)]
struct JobContract {
    /// Outputs produced by this job (for dependent jobs).
    outputs: HashMap<String, String>,

    /// Input parameters passed via `workflow_call`.
    with: Option<ContextValue>,

    /// Secrets available to this job.
    secrets: Option<ContextValue>,
}

impl Job {
    /// Creates a job with its name, runner, steps, and dependencies.
    pub fn new(
        name: Option<String>,
        runs_on: Option<String>,
        steps: Vec<Step>,
        needs: Vec<String>,
    ) -> Self {
        Self {
            identity: JobIdentity { name, needs },
            policy: JobExecutionPolicy::default(),
            runtime: JobRuntime {
                runs_on,
                steps,
                ..JobRuntime::default()
            },
            contract: JobContract::default(),
        }
    }

    /// Sets the condition that controls whether this job runs.
    pub fn with_if_condition(self, condition: Option<String>) -> Self {
        Self {
            policy: JobExecutionPolicy {
                r#if: condition,
                ..self.policy
            },
            ..self
        }
    }

    /// Sets the matrix strategy for this job.
    pub fn with_strategy(self, strategy: Option<JobStrategy>) -> Self {
        Self {
            policy: JobExecutionPolicy {
                strategy,
                ..self.policy
            },
            ..self
        }
    }

    /// Sets the environment variables scoped to this job.
    pub fn with_env(self, env: HashMap<String, String>) -> Self {
        Self {
            runtime: JobRuntime {
                env,
                ..self.runtime
            },
            ..self
        }
    }

    /// Sets the container in which this job runs.
    pub fn with_container(self, container: Option<ContainerSpecification>) -> Self {
        Self {
            runtime: JobRuntime {
                container,
                ..self.runtime
            },
            ..self
        }
    }

    /// Sets the service containers available to this job.
    pub fn with_services(self, services: HashMap<String, ContainerSpecification>) -> Self {
        Self {
            runtime: JobRuntime {
                services,
                ..self.runtime
            },
            ..self
        }
    }

    /// Sets the outputs produced for dependent jobs.
    pub fn with_outputs(self, outputs: HashMap<String, String>) -> Self {
        Self {
            contract: JobContract {
                outputs,
                ..self.contract
            },
            ..self
        }
    }

    /// Sets the input parameters passed to this job.
    pub fn with_inputs(self, inputs: Option<ContextValue>) -> Self {
        Self {
            contract: JobContract {
                with: inputs,
                ..self.contract
            },
            ..self
        }
    }

    /// Sets the secrets available to this job.
    pub fn with_secrets(self, secrets: Option<ContextValue>) -> Self {
        Self {
            contract: JobContract {
                secrets,
                ..self.contract
            },
            ..self
        }
    }

    /// Sets the maximum runtime for this job in minutes.
    pub fn with_timeout_minutes(self, timeout_minutes: Option<f64>) -> Self {
        Self {
            policy: JobExecutionPolicy {
                timeout_minutes,
                ..self.policy
            },
            ..self
        }
    }

    /// Sets whether this job may fail without stopping the workflow.
    pub fn with_continue_on_error(self, value: Option<String>) -> Self {
        Self {
            policy: JobExecutionPolicy {
                continue_on_error: value,
                ..self.policy
            },
            ..self
        }
    }

    /// Sets the token permissions for this job.
    pub fn with_permissions(self, permissions: Option<TokenPermissions>) -> Self {
        Self {
            policy: JobExecutionPolicy {
                permissions,
                ..self.policy
            },
            ..self
        }
    }

    /// Sets the concurrency settings for this job.
    pub fn with_concurrency(self, concurrency: Option<ConcurrencyGroup>) -> Self {
        Self {
            policy: JobExecutionPolicy {
                concurrency,
                ..self.policy
            },
            ..self
        }
    }

    /// Returns the job's display name.
    pub fn name(&self) -> Option<&str> {
        self.identity.name.as_deref()
    }

    /// Returns the machine type assigned to this job.
    pub fn runs_on(&self) -> Option<&str> {
        self.runtime.runs_on.as_deref()
    }

    /// Returns the steps executed by this job.
    pub fn steps(&self) -> &[Step] {
        &self.runtime.steps
    }

    /// Returns the identifiers of jobs that must complete first.
    pub fn needs(&self) -> &[String] {
        &self.identity.needs
    }

    /// Returns this job's run condition.
    pub fn r#if(&self) -> Option<&str> {
        self.policy.r#if.as_deref()
    }

    /// Returns the condition controlling whether this job runs.
    pub fn if_condition(&self) -> Option<&str> {
        self.policy.r#if.as_deref()
    }

    /// Returns this job's matrix strategy.
    pub fn strategy(&self) -> Option<&JobStrategy> {
        self.policy.strategy.as_ref()
    }

    /// Returns the environment variables scoped to this job.
    pub fn env(&self) -> &HashMap<String, String> {
        &self.runtime.env
    }

    /// Returns the container specification for this job.
    pub fn container(&self) -> Option<&ContainerSpecification> {
        self.runtime.container.as_ref()
    }

    /// Returns the service container specifications for this job.
    pub fn services(&self) -> &HashMap<String, ContainerSpecification> {
        &self.runtime.services
    }

    /// Returns the outputs produced by this job.
    pub fn outputs(&self) -> &HashMap<String, String> {
        &self.contract.outputs
    }

    /// Returns the input parameters passed to this job.
    pub fn with(&self) -> Option<&ContextValue> {
        self.contract.with.as_ref()
    }

    /// Returns the secrets available to this job.
    pub fn secrets(&self) -> Option<&ContextValue> {
        self.contract.secrets.as_ref()
    }

    /// Returns the maximum runtime for this job in minutes.
    pub fn timeout_minutes(&self) -> Option<f64> {
        self.policy.timeout_minutes
    }

    /// Returns whether this job may continue after failure.
    pub fn continue_on_error(&self) -> Option<&str> {
        self.policy.continue_on_error.as_deref()
    }

    /// Returns the token permissions for this job.
    pub fn permissions(&self) -> Option<&TokenPermissions> {
        self.policy.permissions.as_ref()
    }

    /// Returns the concurrency settings for this job.
    pub fn concurrency(&self) -> Option<&ConcurrencyGroup> {
        self.policy.concurrency.as_ref()
    }

    /// Returns whether this job depends directly on `job_id`.
    pub fn depends_on(&self, job_id: &str) -> bool {
        self.identity
            .needs
            .iter()
            .any(|dependency| dependency == job_id)
    }

    /// Returns whether a failed step should not stop this job.
    pub fn continues_after_failure(&self) -> bool {
        self.policy
            .continue_on_error
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
        )
        .with_if_condition(Some("always()".into()))
        .with_env(HashMap::from([("KEY".into(), "value".into())]))
        .with_outputs(HashMap::from([("output".into(), "value".into())]))
        .with_inputs(Some(ContextValue::mapping([(
            "token".to_owned(),
            ContextValue::text("value"),
        )])))
        .with_secrets(Some(ContextValue::empty_mapping()))
        .with_timeout_minutes(Some(10.0))
        .with_continue_on_error(Some("true".into()));

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
        let job = Job::new(None, None, Vec::new(), vec!["setup".into()])
            .with_continue_on_error(Some("true".into()));

        assert!(job.depends_on("setup"));
        assert!(!job.depends_on("build"));
        assert!(job.continues_after_failure());
    }

    #[test]
    fn builders_set_optional_infrastructure_sections() {
        let container = ContainerSpecification::new("node:20");
        let service = ContainerSpecification::new("postgres:16");
        let job = Job::default()
            .with_strategy(Some(JobStrategy::new(None, true, Some(2))))
            .with_container(Some(container.clone()))
            .with_services(HashMap::from([("db".into(), service.clone())]))
            .with_permissions(Some(TokenPermissions::new()))
            .with_concurrency(Some(ConcurrencyGroup::new("job", Some(false))));

        assert_eq!(job.strategy(), Some(&JobStrategy::new(None, true, Some(2))));
        assert_eq!(job.container(), Some(&container));
        assert_eq!(job.services()["db"], service);
        assert_eq!(job.permissions(), Some(&TokenPermissions::new()));
        assert_eq!(
            job.concurrency(),
            Some(&ConcurrencyGroup::new("job", Some(false)))
        );
    }
    #[test]
    fn stores_fields_in_focused_sections() {
        let container = ContainerSpecification::new("node:20");
        let service = ContainerSpecification::new("postgres:16");
        let strategy = JobStrategy::new(None, true, Some(2));
        let job = Job::new(
            Some("Build".into()),
            Some("ubuntu".into()),
            Vec::new(),
            vec!["setup".into()],
        )
        .with_if_condition(Some("always()".into()))
        .with_strategy(Some(strategy.clone()))
        .with_env(HashMap::from([("KEY".into(), "value".into())]))
        .with_container(Some(container.clone()))
        .with_services(HashMap::from([("db".into(), service.clone())]))
        .with_outputs(HashMap::from([("output".into(), "value".into())]))
        .with_inputs(Some(ContextValue::text("input")))
        .with_secrets(Some(ContextValue::text("secret")))
        .with_timeout_minutes(Some(10.0))
        .with_continue_on_error(Some("true".into()))
        .with_permissions(Some(TokenPermissions::new()))
        .with_concurrency(Some(ConcurrencyGroup::new("job", Some(false))));

        assert_eq!(job.identity.name.as_deref(), Some("Build"));
        assert_eq!(job.identity.needs, vec!["setup"]);
        assert_eq!(job.policy.r#if.as_deref(), Some("always()"));
        assert_eq!(job.policy.strategy, Some(strategy));
        assert_eq!(job.policy.timeout_minutes, Some(10.0));
        assert_eq!(job.policy.continue_on_error.as_deref(), Some("true"));
        assert!(job.policy.permissions.is_some());
        assert!(job.policy.concurrency.is_some());
        assert_eq!(job.runtime.runs_on.as_deref(), Some("ubuntu"));
        assert!(job.runtime.steps.is_empty());
        assert_eq!(job.runtime.env["KEY"], "value");
        assert_eq!(job.runtime.container, Some(container));
        assert_eq!(job.runtime.services["db"], service);
        assert_eq!(job.contract.outputs["output"], "value");
        assert_eq!(job.contract.with, Some(ContextValue::text("input")));
        assert_eq!(job.contract.secrets, Some(ContextValue::text("secret")));
    }
}
