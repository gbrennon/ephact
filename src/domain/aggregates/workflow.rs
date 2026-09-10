use std::collections::HashMap;

use crate::domain::{
    entities::Job,
    value_objects::{ConcurrencyGroup, ExecutionDefaults, TokenPermissions, WorkflowTrigger},
};

/// Represents a parsed GitHub Actions workflow file.
///
/// Maps to the top-level structure of a workflow YAML file.
/// Supports all standard fields including `name`, `on`, `env`, `jobs`,
/// `defaults`, and `permissions`.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
///
/// use ephact::domain::{aggregates::Workflow, value_objects::WorkflowTrigger};
///
/// let workflow = Workflow::new(
///     Some("CI".to_owned()),
///     None,
///     WorkflowTrigger::Single("push".to_owned()),
///     HashMap::new(),
///     HashMap::new(),
///     None,
///     None,
///     None,
/// );
///
/// assert_eq!(workflow.name(), Some("CI"));
/// assert!(workflow.trigger().has_event("push"));
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Workflow {
    /// The name of the workflow displayed on GitHub's actions page.
    name: Option<String>,

    /// The name of the workflow file (set after parsing, not from YAML).
    file: Option<String>,

    /// The event(s) that trigger this workflow.
    trigger: WorkflowTrigger,

    /// Environment variables available to all jobs and steps.
    env: HashMap<String, String>,

    /// The jobs that make up this workflow.
    jobs: HashMap<String, Job>,

    /// Default settings applied to all jobs in the workflow.
    defaults: Option<ExecutionDefaults>,

    /// TokenPermissions for the `GITHUB_TOKEN`.
    permissions: Option<TokenPermissions>,

    /// ConcurrencyGroup group to limit parallel runs.
    concurrency: Option<ConcurrencyGroup>,
}

impl Workflow {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: Option<String>,
        file: Option<String>,
        trigger: WorkflowTrigger,
        env: HashMap<String, String>,
        jobs: HashMap<String, Job>,
        defaults: Option<ExecutionDefaults>,
        permissions: Option<TokenPermissions>,
        concurrency: Option<ConcurrencyGroup>,
    ) -> Self {
        Self {
            name,
            file,
            trigger,
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

    pub fn trigger(&self) -> &WorkflowTrigger {
        &self.trigger
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn jobs(&self) -> &HashMap<String, Job> {
        &self.jobs
    }

    pub fn defaults(&self) -> Option<&ExecutionDefaults> {
        self.defaults.as_ref()
    }

    pub fn permissions(&self) -> Option<&TokenPermissions> {
        self.permissions.as_ref()
    }

    pub fn concurrency(&self) -> Option<&ConcurrencyGroup> {
        self.concurrency.as_ref()
    }
    /// Returns whether this workflow declares the given trigger event.
    pub fn triggers_on(&self, event_name: &str) -> bool {
        self.trigger.has_event(event_name)
    }

    /// Returns the job identified by `job_id`.
    pub fn job_named(&self, job_id: &str) -> Option<&Job> {
        self.jobs.get(job_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new_and_with_file_preserve_fields() {
        let workflow = Workflow::new(
            Some("CI".into()),
            None,
            WorkflowTrigger::default(),
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
        assert_eq!(workflow.trigger(), &WorkflowTrigger::default());
        assert!(workflow.permissions().is_none());
        assert!(workflow.concurrency().is_none());
    }
    #[test]
    fn exposes_trigger_and_named_job_behavior() {
        let job = Job::default();
        let workflow = Workflow::new(
            Some("CI".into()),
            None,
            WorkflowTrigger::Single("push".into()),
            HashMap::new(),
            HashMap::from([("build".into(), job)]),
            None,
            None,
            None,
        );

        assert!(workflow.triggers_on("push"));
        assert!(!workflow.triggers_on("pull_request"));
        assert!(workflow.job_named("build").is_some());
        assert!(workflow.job_named("missing").is_none());
    }
}
