use std::collections::HashMap;

use crate::{
    traits::NetworkCommandClassifier,
    value_objects::{StepType, step_network_policy::StepNetworkPolicy},
};

/// A step in a workflow job.
///
/// Steps can be shell commands (`run`) or actions (`uses`).
/// They execute sequentially within a job and can be gated with `if`.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
///
/// use ephact_domain::entities::Step;
///
/// let step = Step::new(
///     None,
///     None,
///     Some("echo hello".to_owned()),
///     None,
/// )
/// .with_shell(Some("bash".to_owned()));
///
/// assert_eq!(step.run(), Some("echo hello"));
/// assert!(step.is_run_step());
/// ```
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Step {
    /// An identifier for the step (used for output references).
    id: Option<String>,

    /// A display name for the step.
    name: Option<String>,

    /// An expression that determines whether the step runs.
    r#if: Option<String>,

    /// Shell command(s) to execute.
    run: Option<String>,

    /// The shell to use for `run` commands.
    shell: Option<String>,

    /// The working directory for `run` commands.
    working_directory: Option<String>,

    /// An action reference (`./`, `docker://`, or `owner/repo@ref`).
    uses: Option<String>,

    /// Input parameters for a `uses` action.
    with: HashMap<String, String>,

    /// Environment variables scoped to this step.
    env: HashMap<String, String>,

    /// Whether to continue the job even if this step fails.
    continue_on_error: Option<String>,

    /// Maximum number of minutes to let the step run.
    timeout_minutes: Option<f64>,
}

impl Step {
    pub fn new(
        id: Option<String>,
        name: Option<String>,
        run: Option<String>,
        uses: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            r#if: None,
            run,
            shell: None,
            working_directory: None,
            uses,
            with: HashMap::new(),
            env: HashMap::new(),
            continue_on_error: None,
            timeout_minutes: None,
        }
    }

    pub fn with_if_condition(mut self, condition: Option<String>) -> Self {
        self.r#if = condition;
        self
    }

    pub fn with_shell(mut self, shell: Option<String>) -> Self {
        self.shell = shell;
        self
    }

    pub fn with_working_directory(mut self, working_directory: Option<String>) -> Self {
        self.working_directory = working_directory;
        self
    }

    pub fn with_inputs(mut self, inputs: HashMap<String, String>) -> Self {
        self.with = inputs;
        self
    }

    pub fn with_env(mut self, env: HashMap<String, String>) -> Self {
        self.env = env;
        self
    }

    pub fn with_continue_on_error(mut self, value: Option<String>) -> Self {
        self.continue_on_error = value;
        self
    }

    pub fn with_timeout_minutes(mut self, timeout_minutes: Option<f64>) -> Self {
        self.timeout_minutes = timeout_minutes;
        self
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the label used for step progress and summary output.
    pub fn display_name(&self) -> &str {
        self.name
            .as_deref()
            .or(self.id.as_deref())
            .or(self.run.as_deref())
            .or(self.uses.as_deref())
            .unwrap_or("unnamed step")
    }

    pub fn r#if(&self) -> Option<&str> {
        self.r#if.as_deref()
    }

    pub fn if_condition(&self) -> Option<&str> {
        self.r#if.as_deref()
    }

    pub fn shell(&self) -> Option<&str> {
        self.shell.as_deref()
    }

    pub fn working_directory(&self) -> Option<&str> {
        self.working_directory.as_deref()
    }

    pub fn with(&self) -> &HashMap<String, String> {
        &self.with
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn continue_on_error(&self) -> Option<&str> {
        self.continue_on_error.as_deref()
    }

    pub fn timeout_minutes(&self) -> Option<f64> {
        self.timeout_minutes
    }

    /// Returns the `run` command if this is a run step.
    pub fn run(&self) -> Option<&str> {
        self.run.as_deref()
    }

    /// Returns the `uses` action reference if this is a uses step.
    pub fn uses(&self) -> Option<&str> {
        self.uses.as_deref()
    }

    /// Returns `true` if this is a run step (has `run`, no `uses`).
    pub fn is_run_step(&self) -> bool {
        self.run.is_some() && self.uses.is_none()
    }

    /// Returns `true` if this is a uses step (has `uses`, no `run`).
    pub fn is_uses_step(&self) -> bool {
        self.uses.is_some() && self.run.is_none()
    }

    /// Returns the effective shell for this step.
    ///
    /// Falls back to the default shell if none is specified.
    pub fn effective_shell<'a>(&'a self, default_shell: &'a str) -> &'a str {
        self.shell.as_deref().unwrap_or(default_shell)
    }

    /// Classifies this step: `Run` for shell commands, `Composite` for local
    /// (`./`) actions, `Uses` for other action references, and `Invalid` when
    /// the step defines neither `run` nor `uses`.
    pub fn step_type(&self) -> StepType {
        if self.run.is_some() {
            StepType::Run
        } else if self.uses.as_deref().is_some_and(|u| u.starts_with("./")) {
            StepType::Composite
        } else if self.uses.is_some() {
            StepType::Uses
        } else {
            StepType::Invalid
        }
    }

    /// Returns `true` when `continue-on-error` is set to a truthy value.
    ///
    /// The workflow parser preserves the raw scalar, so values like `True` are
    /// matched case-insensitively.
    pub fn continues_on_error(&self) -> bool {
        self.continue_on_error
            .as_deref()
            .is_some_and(|v| v.eq_ignore_ascii_case("true"))
    }

    pub fn network_access_reason(
        &self,
        classifier: &dyn NetworkCommandClassifier,
    ) -> Option<&'static str> {
        let script = self.run()?;
        StepNetworkPolicy::new(script).network_access_reason(classifier)
    }

    pub fn network_policy_violation(
        &self,
        classifier: &dyn NetworkCommandClassifier,
    ) -> Option<&'static str> {
        let script = self.run()?;
        StepNetworkPolicy::new(script).network_policy_violation(classifier)
    }
}

#[cfg(test)]
mod tests;
