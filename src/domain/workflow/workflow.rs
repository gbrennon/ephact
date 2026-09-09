use std::collections::HashMap;

use serde::Deserialize;

use super::{Concurrency, Defaults, Job, On, Permissions};

/// Represents a parsed GitHub Actions workflow file.
///
/// Maps to the top-level structure of a workflow YAML file.
/// Supports all standard fields including `name`, `on`, `env`, `jobs`,
/// `defaults`, and `permissions`.
///
/// # Examples
///
/// ```
/// use ephact::domain::workflow::Workflow;
///
/// let yaml = r#"
/// name: CI
/// on: push
/// jobs:
///   build:
///     runs-on: ubuntu-latest
///     steps:
///       - run: echo hello
/// "#;
/// let wf: Workflow = serde_yaml::from_str(yaml).unwrap();
/// assert_eq!(wf.name(), Some("CI"));
/// assert_eq!(wf.jobs().len(), 1);
/// ```
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
#[serde(rename_all = "kebab-case")]
pub struct Workflow {
    /// The name of the workflow displayed on GitHub's actions page.
    name: Option<String>,

    /// The name of the workflow file (set after parsing, not from YAML).
    #[serde(skip)]
    file: Option<String>,

    /// The event(s) that trigger this workflow.
    #[serde(default)]
    on: On,

    /// Environment variables available to all jobs and steps.
    #[serde(default)]
    env: HashMap<String, String>,

    /// The jobs that make up this workflow.
    #[serde(default)]
    jobs: HashMap<String, Job>,

    /// Default settings applied to all jobs in the workflow.
    #[serde(default)]
    defaults: Option<Defaults>,

    /// Permissions for the `GITHUB_TOKEN`.
    #[serde(default)]
    permissions: Option<Permissions>,

    /// Concurrency group to limit parallel runs.
    #[serde(default)]
    concurrency: Option<Concurrency>,
}

impl Workflow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: Option<String>,
        file: Option<String>,
        on: On,
        env: HashMap<String, String>,
        jobs: HashMap<String, Job>,
        defaults: Option<Defaults>,
        permissions: Option<Permissions>,
        concurrency: Option<Concurrency>,
    ) -> Self {
        Self {
            name,
            file,
            on,
            env,
            jobs,
            defaults,
            permissions,
            concurrency,
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn file(&self) -> Option<&str> {
        self.file.as_deref()
    }

    pub fn with_file(mut self, file: impl Into<String>) -> Self {
        self.file = Some(file.into());
        self
    }

    pub fn on(&self) -> &On {
        &self.on
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn jobs(&self) -> &HashMap<String, Job> {
        &self.jobs
    }

    pub fn defaults(&self) -> Option<&Defaults> {
        self.defaults.as_ref()
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
    fn parse_minimal_workflow() {
        let yaml = "on: push\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      - run: echo hello\n";
        let wf: Workflow = serde_yaml::from_str(yaml).unwrap();
        assert!(wf.name().is_none());
        assert_eq!(wf.jobs().len(), 1);
        assert!(wf.jobs().contains_key("build"));
    }

    #[test]
    fn parse_workflow_with_name_and_env() {
        let yaml = r#"
name: CI
on: [push, pull_request]
env:
  RUST_BACKTRACE: "1"
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - run: cargo test
"#;
        let wf: Workflow = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(wf.name(), Some("CI"));
        assert_eq!(
            wf.env().get("RUST_BACKTRACE").map(|s| s.as_str()),
            Some("1")
        );
    }

    #[test]
    fn parse_workflow_with_defaults() {
        let yaml = r#"
on: push
defaults:
  run:
    shell: bash
    working-directory: ./src
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - run: make
"#;
        let wf: Workflow = serde_yaml::from_str(yaml).unwrap();
        let defaults = wf.defaults().unwrap();
        let run_defaults = defaults.run().unwrap();
        assert_eq!(run_defaults.shell(), Some("bash"));
        assert_eq!(run_defaults.working_directory(), Some("./src"));
    }
    #[test]
    fn new_and_with_file_preserve_fields() {
        let workflow = Workflow::new(
            Some("CI".into()),
            None,
            On::default(),
            HashMap::from([("KEY".into(), "value".into())]),
            HashMap::new(),
            None,
            None,
            None,
        )
        .with_file("workflow.yml");

        assert_eq!(workflow.name(), Some("CI"));
        assert_eq!(workflow.file(), Some("workflow.yml"));
        assert_eq!(workflow.env()["KEY"], "value");
        assert!(workflow.jobs().is_empty());
        assert!(workflow.defaults().is_none());
        assert_eq!(workflow.on(), &On::default());
        assert!(workflow.permissions().is_none());
        assert!(workflow.concurrency().is_none());
    }
}
