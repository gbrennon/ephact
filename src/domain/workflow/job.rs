use std::collections::HashMap;

use serde::Deserialize;

use super::{Concurrency, ContainerConfig, JobNeedsVisitor, Permissions, Step, Strategy};

/// A job in a GitHub Actions workflow.
///
/// Jobs run in parallel by default but can be sequenced with `needs`.
/// Each job runs on a fresh virtual environment specified by `runs_on`.
///
/// # Examples
///
/// ```
/// use ephact::domain::workflow::Job;
///
/// let yaml = r#"
/// runs-on: ubuntu-latest
/// steps:
///   - run: echo hello
/// "#;
/// let job: Job = serde_yaml::from_str(yaml).unwrap();
/// assert_eq!(job.runs_on(), Some("ubuntu-latest"));
/// ```
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Job {
    /// The name of the job displayed on GitHub.
    name: Option<String>,

    /// The type of machine to run the job on (e.g. `ubuntu-latest`).
    #[serde(rename = "runs-on")]
    runs_on: Option<String>,

    /// The sequence of steps to execute.
    #[serde(default)]
    steps: Vec<Step>,

    /// Jobs that must complete successfully before this job runs.
    #[serde(default, deserialize_with = "JobNeedsVisitor::deserialize")]
    needs: Vec<String>,

    /// An expression that determines whether the job runs.
    #[serde(rename = "if")]
    r#if: Option<String>,

    /// A matrix strategy to generate multiple job runs.
    #[serde(default)]
    strategy: Option<Strategy>,

    /// Environment variables scoped to this job.
    #[serde(default)]
    env: HashMap<String, String>,

    /// Container to run the job inside.
    #[serde(default)]
    container: Option<ContainerConfig>,

    /// Service containers to run alongside the job.
    #[serde(default)]
    services: HashMap<String, ContainerConfig>,

    /// Outputs produced by this job (for dependent jobs).
    #[serde(default)]
    outputs: HashMap<String, String>,

    /// Input parameters passed via `workflow_call`.
    #[serde(default)]
    with: Option<serde_yaml::Value>,

    /// Secrets available to this job.
    #[serde(default)]
    secrets: Option<serde_yaml::Value>,

    /// Maximum number of minutes to let the job run.
    #[serde(rename = "timeout-minutes")]
    timeout_minutes: Option<f64>,

    /// Whether to continue the workflow even if this job fails.
    #[serde(rename = "continue-on-error")]
    continue_on_error: Option<String>,

    /// Permissions override for this job.
    #[serde(default)]
    permissions: Option<Permissions>,

    /// Concurrency override for this job.
    #[serde(default)]
    concurrency: Option<Concurrency>,
}

impl Job {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: Option<String>,
        runs_on: Option<String>,
        steps: Vec<Step>,
        needs: Vec<String>,
        r#if: Option<String>,
        strategy: Option<Strategy>,
        env: HashMap<String, String>,
        container: Option<ContainerConfig>,
        services: HashMap<String, ContainerConfig>,
        outputs: HashMap<String, String>,
        with: Option<serde_yaml::Value>,
        secrets: Option<serde_yaml::Value>,
        timeout_minutes: Option<f64>,
        continue_on_error: Option<String>,
        permissions: Option<Permissions>,
        concurrency: Option<Concurrency>,
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

    pub fn strategy(&self) -> Option<&Strategy> {
        self.strategy.as_ref()
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn container(&self) -> Option<&ContainerConfig> {
        self.container.as_ref()
    }

    pub fn services(&self) -> &HashMap<String, ContainerConfig> {
        &self.services
    }

    pub fn outputs(&self) -> &HashMap<String, String> {
        &self.outputs
    }

    pub fn with(&self) -> Option<&serde_yaml::Value> {
        self.with.as_ref()
    }

    pub fn secrets(&self) -> Option<&serde_yaml::Value> {
        self.secrets.as_ref()
    }

    pub fn timeout_minutes(&self) -> Option<f64> {
        self.timeout_minutes
    }

    pub fn continue_on_error(&self) -> Option<&str> {
        self.continue_on_error.as_deref()
    }

    pub fn permissions(&self) -> Option<&Permissions> {
        self.permissions.as_ref()
    }

    pub fn concurrency(&self) -> Option<&Concurrency> {
        self.concurrency.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_minimal_job() {
        let yaml = "runs-on: ubuntu-latest\nsteps:\n  - run: echo hello\n";
        let job: Job = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(job.runs_on(), Some("ubuntu-latest"));
        assert_eq!(job.steps().len(), 1);
    }

    #[test]
    fn parse_job_with_needs_and_if() {
        let yaml = r#"
runs-on: ubuntu-latest
needs: [build, lint]
if: github.ref == 'refs/heads/main'
steps:
  - run: echo deploy
"#;
        let job: Job = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(job.needs(), &["build", "lint"]);
        assert_eq!(job.r#if(), Some("github.ref == 'refs/heads/main'"));
    }

    #[test]
    fn parse_job_with_container() {
        let yaml = r#"
runs-on: ubuntu-latest
container:
  image: node:18
  env:
    NODE_ENV: test
steps:
  - run: npm test
"#;
        let job: Job = serde_yaml::from_str(yaml).unwrap();
        let container = job.container().unwrap();
        assert_eq!(container.image(), "node:18");
        assert_eq!(
            container.env().get("NODE_ENV").map(|s| s.as_str()),
            Some("test")
        );
    }

    #[test]
    fn parse_job_with_timeout() {
        let yaml = r#"
runs-on: ubuntu-latest
timeout-minutes: 30
steps:
  - run: sleep 9999
"#;
        let job: Job = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(job.timeout_minutes(), Some(30.0));
    }
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
            Some(serde_yaml::Value::Null),
            Some(serde_yaml::Value::Null),
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
        assert!(job.with().is_some());
        assert!(job.secrets().is_some());
        assert_eq!(job.timeout_minutes(), Some(10.0));
        assert_eq!(job.continue_on_error(), Some("true"));
        assert!(job.strategy().is_none());
        assert!(job.container().is_none());
        assert!(job.permissions().is_none());
        assert!(job.concurrency().is_none());
    }
}
