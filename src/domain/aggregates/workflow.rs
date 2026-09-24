use std::collections::HashMap;

use crate::domain::{
    entities::Job,
    value_objects::{
        ConcurrencyGroup, ExecutionDefaults, TokenPermissions, TriggerKind, WorkflowTrigger,
    },
};

/// Represents a parsed workflow file.
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
/// use ephact::domain::{
///     aggregates::Workflow,
///     value_objects::{TriggerKind, WorkflowTrigger},
/// };
///
/// let workflow = Workflow::new(
///     Some("CI".to_owned()),
///     vec![WorkflowTrigger::Push(None)],
///     HashMap::new(),
///     HashMap::new(),
/// );
///
/// assert_eq!(workflow.name(), Some("CI"));
/// assert!(workflow.triggers_on(TriggerKind::Push));
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Workflow {
    /// The display name of the workflow.
    name: Option<String>,

    /// The name of the workflow file (set after parsing, not from YAML).
    file: Option<String>,

    /// The triggers that activate this workflow.
    trigger: Vec<WorkflowTrigger>,

    /// Environment variables available to all jobs and steps.
    env: HashMap<String, String>,

    /// The jobs that make up this workflow.
    jobs: HashMap<String, Job>,

    /// Default settings applied to all jobs in the workflow.
    defaults: Option<ExecutionDefaults>,

    /// TokenPermissions for the workflow token.
    permissions: Option<TokenPermissions>,

    /// ConcurrencyGroup group to limit parallel runs.
    concurrency: Option<ConcurrencyGroup>,
}

impl Workflow {
    pub fn new(
        name: Option<String>,
        trigger: Vec<WorkflowTrigger>,
        env: HashMap<String, String>,
        jobs: HashMap<String, Job>,
    ) -> Self {
        Self {
            name,
            file: None,
            trigger,
            env,
            jobs,
            defaults: None,
            permissions: None,
            concurrency: None,
        }
    }

    pub fn with_defaults(mut self, defaults: Option<ExecutionDefaults>) -> Self {
        self.defaults = defaults;
        self
    }

    pub fn with_permissions(mut self, permissions: Option<TokenPermissions>) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn with_concurrency(mut self, concurrency: Option<ConcurrencyGroup>) -> Self {
        self.concurrency = concurrency;
        self
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

    pub fn trigger(&self) -> &[WorkflowTrigger] {
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
    pub fn triggers_on(&self, kind: TriggerKind) -> bool {
        self.trigger.iter().any(|trigger| trigger.kind() == kind)
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
            vec![WorkflowTrigger::Push(None)],
            HashMap::from([("KEY".into(), "value".into())]),
            HashMap::new(),
        )
        .with_file("workflow.yml");

        assert_eq!(workflow.name(), Some("CI"));
        assert_eq!(workflow.file(), Some("workflow.yml"));
        assert_eq!(workflow.env()["KEY"], "value");
        assert!(workflow.jobs().is_empty());
        assert!(workflow.defaults().is_none());
        assert_eq!(workflow.trigger(), &[WorkflowTrigger::Push(None)]);
        assert!(workflow.permissions().is_none());
        assert!(workflow.concurrency().is_none());
    }
    #[test]
    fn exposes_trigger_and_named_job_behavior() {
        let job = Job::default();
        let workflow = Workflow::new(
            Some("CI".into()),
            vec![WorkflowTrigger::Push(None)],
            HashMap::new(),
            HashMap::from([("build".into(), job)]),
        );

        assert!(workflow.triggers_on(TriggerKind::Push));
        assert!(!workflow.triggers_on(TriggerKind::PullRequest));
        assert!(workflow.job_named("build").is_some());
        assert!(workflow.job_named("missing").is_none());
    }
}
