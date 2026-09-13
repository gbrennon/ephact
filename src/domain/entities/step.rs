use std::collections::HashMap;

use crate::domain::value_objects::StepType;

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
/// use ephact::domain::entities::Step;
///
/// let step = Step::new(
///     None,
///     None,
///     None,
///     Some("echo hello".to_owned()),
///     Some("bash".to_owned()),
///     None,
///     None,
///     HashMap::new(),
///     HashMap::new(),
///     None,
///     None,
/// );
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
}

#[cfg(test)]
mod tests {
    use super::*;

    fn step(run: Option<&str>, shell: Option<&str>, uses: Option<&str>) -> Step {
        Step::new(
            None,
            None,
            None,
            run.map(str::to_owned),
            shell.map(str::to_owned),
            None,
            uses.map(str::to_owned),
            HashMap::new(),
            HashMap::new(),
            None,
            None,
        )
    }

    #[test]
    fn a_step_with_a_script_is_a_run_step() {
        let step = step(Some("cargo test"), None, None);

        assert!(step.is_run_step());
        assert!(!step.is_uses_step());
        assert_eq!(step.step_type(), StepType::Run);
    }

    #[test]
    fn a_step_with_an_action_reference_is_a_uses_step() {
        let step = step(None, None, Some("actions/checkout@v4"));

        assert!(step.is_uses_step());
        assert!(!step.is_run_step());
        assert_eq!(step.step_type(), StepType::Uses);
    }

    #[test]
    fn a_local_action_reference_is_a_composite_step() {
        assert_eq!(
            step(None, None, Some("./action")).step_type(),
            StepType::Composite
        );
    }

    #[test]
    fn a_step_without_a_script_or_action_is_invalid() {
        assert_eq!(step(None, None, None).step_type(), StepType::Invalid);
    }

    #[test]
    fn effective_shell_uses_default_when_not_set() {
        assert_eq!(
            step(Some("echo hello"), None, None).effective_shell("bash"),
            "bash"
        );
    }

    #[test]
    fn effective_shell_uses_step_shell_when_set() {
        assert_eq!(
            step(Some("echo hello"), Some("pwsh"), None).effective_shell("bash"),
            "pwsh"
        );
    }

    #[test]
    fn continues_on_error_matches_truthy_scalars_case_insensitively() {
        let with_flag = |flag: &str| {
            Step::new(
                None,
                None,
                None,
                Some("echo".to_owned()),
                None,
                None,
                None,
                HashMap::new(),
                HashMap::new(),
                Some(flag.to_owned()),
                None,
            )
        };

        assert!(with_flag("true").continues_on_error());
        assert!(with_flag("True").continues_on_error());
        assert!(!with_flag("false").continues_on_error());
        assert!(!step(Some("echo"), None, None).continues_on_error());
    }

    #[test]
    fn display_name_falls_back_through_id_script_and_action() {
        let named = Step::new(
            Some("step-id".to_owned()),
            Some("Run tests".to_owned()),
            None,
            Some("cargo test".to_owned()),
            None,
            None,
            None,
            HashMap::new(),
            HashMap::new(),
            None,
            None,
        );
        let identified = Step::new(
            Some("step-id".to_owned()),
            None,
            None,
            Some("cargo test".to_owned()),
            None,
            None,
            None,
            HashMap::new(),
            HashMap::new(),
            None,
            None,
        );

        assert_eq!(named.display_name(), "Run tests");
        assert_eq!(identified.display_name(), "step-id");
        assert_eq!(
            step(Some("cargo test"), None, None).display_name(),
            "cargo test"
        );
        assert_eq!(
            step(None, None, Some("./action")).display_name(),
            "./action"
        );
        assert_eq!(step(None, None, None).display_name(), "unnamed step");
    }
}
