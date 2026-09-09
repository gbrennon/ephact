use std::collections::HashMap;

use serde::Deserialize;

use crate::domain::workflow::StepType;

/// A step in a GitHub Actions job.
///
/// Steps can be shell commands (`run`) or actions (`uses`).
/// They execute sequentially within a job and can be gated with `if`.
///
/// # Examples
///
/// ```
/// use ephact::domain::workflow::Step;
///
/// // A run step
/// let yaml = "run: echo hello\nshell: bash\n";
/// let step: Step = serde_yaml::from_str(yaml).unwrap();
/// assert_eq!(step.run(), Some("echo hello"));
///
/// // A uses step
/// let yaml = "uses: actions/checkout@v4\n";
/// let step: Step = serde_yaml::from_str(yaml).unwrap();
/// assert_eq!(step.uses(), Some("actions/checkout@v4"));
/// ```
#[derive(Debug, Clone, Deserialize, PartialEq, Default)]
pub struct Step {
    /// An identifier for the step (used for output references).
    id: Option<String>,

    /// A display name for the step.
    name: Option<String>,

    /// An expression that determines whether the step runs.
    #[serde(rename = "if")]
    r#if: Option<String>,

    /// Shell command(s) to execute.
    #[serde(default)]
    run: Option<String>,

    /// The shell to use for `run` commands.
    #[serde(default)]
    shell: Option<String>,

    /// The working directory for `run` commands.
    #[serde(rename = "working-directory")]
    working_directory: Option<String>,

    /// An action reference (`./`, `docker://`, or `owner/repo@ref`).
    #[serde(default)]
    uses: Option<String>,

    /// Input parameters for a `uses` action.
    #[serde(default)]
    with: HashMap<String, String>,

    /// Environment variables scoped to this step.
    #[serde(default)]
    env: HashMap<String, String>,

    /// Whether to continue the job even if this step fails.
    #[serde(rename = "continue-on-error")]
    continue_on_error: Option<String>,

    /// Maximum number of minutes to let the step run.
    #[serde(rename = "timeout-minutes")]
    timeout_minutes: Option<f64>,
}

impl Step {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        id: Option<String>,
        name: Option<String>,
        r#if: Option<String>,
        run: Option<String>,
        shell: Option<String>,
        working_directory: Option<String>,
        uses: Option<String>,
        with: HashMap<String, String>,
        env: HashMap<String, String>,
        continue_on_error: Option<String>,
        timeout_minutes: Option<f64>,
    ) -> Self {
        Self {
            id,
            name,
            r#if,
            run,
            shell,
            working_directory,
            uses,
            with,
            env,
            continue_on_error,
            timeout_minutes,
        }
    }

    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_run_step() {
        let yaml = "run: cargo test\n";
        let step: Step = serde_yaml::from_str(yaml).unwrap();
        assert!(step.is_run_step());
        assert!(!step.is_uses_step());
        assert_eq!(step.run(), Some("cargo test"));
    }

    #[test]
    fn parse_uses_step() {
        let yaml = "uses: actions/checkout@v4\n";
        let step: Step = serde_yaml::from_str(yaml).unwrap();
        assert!(step.is_uses_step());
        assert!(!step.is_run_step());
        assert_eq!(step.uses(), Some("actions/checkout@v4"));
    }

    #[test]
    fn parse_step_with_all_fields() {
        let yaml = r#"
id: test-step
name: Run tests
if: success()
run: cargo test
shell: bash
working-directory: ./src
env:
  RUST_LOG: debug
continue-on-error: true
timeout-minutes: 10
"#;
        let step: Step = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(step.id(), Some("test-step"));
        assert_eq!(step.name(), Some("Run tests"));
        assert_eq!(step.r#if(), Some("success()"));
        assert_eq!(step.run(), Some("cargo test"));
        assert_eq!(step.shell(), Some("bash"));
        assert_eq!(step.working_directory(), Some("./src"));
        assert_eq!(step.env().get("RUST_LOG").map(|s| s.as_str()), Some("debug"));
        assert_eq!(step.continue_on_error(), Some("true"));
        assert_eq!(step.timeout_minutes(), Some(10.0));
    }

    #[test]
    fn effective_shell_uses_default_when_not_set() {
        let yaml = "run: echo hello\n";
        let step: Step = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(step.effective_shell("bash"), "bash");
    }

    #[test]
    fn effective_shell_uses_step_shell_when_set() {
        let yaml = "run: echo hello\nshell: pwsh\n";
        let step: Step = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(step.effective_shell("bash"), "pwsh");
    }
}
