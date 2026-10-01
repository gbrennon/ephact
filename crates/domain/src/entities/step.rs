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
mod tests {
    use super::*;

    impl Step {
        fn for_test(run: Option<&str>, shell: Option<&str>, uses: Option<&str>) -> Self {
            Self::new(None, None, run.map(str::to_owned), uses.map(str::to_owned))
                .with_shell(shell.map(str::to_owned))
        }
    }

    #[test]
    fn step_type_returns_run_for_script_step() {
        let step = Step::for_test(Some("cargo test"), None, None);

        assert_eq!(step.step_type(), StepType::Run);
    }

    #[test]
    fn step_type_returns_uses_for_remote_action() {
        let step = Step::for_test(None, None, Some("actions/checkout@v4"));

        assert_eq!(step.step_type(), StepType::Uses);
    }

    #[test]
    fn step_type_returns_composite_for_local_action() {
        let step = Step::for_test(None, None, Some("./action"));

        assert_eq!(step.step_type(), StepType::Composite);
    }

    #[test]
    fn step_type_returns_invalid_without_script_or_action() {
        let step = Step::for_test(None, None, None);

        assert_eq!(step.step_type(), StepType::Invalid);
    }

    #[test]
    fn is_run_step_returns_true_for_script_step() {
        let step = Step::for_test(Some("cargo test"), None, None);

        assert!(step.is_run_step());
    }

    #[test]
    fn is_uses_step_returns_true_for_action_step() {
        let step = Step::for_test(None, None, Some("actions/checkout@v4"));

        assert!(step.is_uses_step());
    }

    #[test]
    fn effective_shell_returns_configured_shell() {
        let step = Step::for_test(Some("echo hello"), Some("pwsh"), None);

        assert_eq!(step.effective_shell("bash"), "pwsh");
    }

    #[test]
    fn continues_on_error_returns_true_for_true_flag() {
        let step = Step::new(None, None, Some("echo".to_owned()), None)
            .with_continue_on_error(Some("true".to_owned()));

        assert!(step.continues_on_error());
    }

    #[test]
    fn continues_on_error_returns_true_for_case_insensitive_true_flag() {
        let step = Step::new(None, None, Some("echo".to_owned()), None)
            .with_continue_on_error(Some("True".to_owned()));

        assert!(step.continues_on_error());
    }

    #[test]
    fn continues_on_error_returns_false_for_false_flag() {
        let step = Step::new(None, None, Some("echo".to_owned()), None)
            .with_continue_on_error(Some("false".to_owned()));

        assert!(!step.continues_on_error());
    }

    #[test]
    fn display_name_returns_explicit_name() {
        let step = Step::new(
            Some("step-id".to_owned()),
            Some("Run tests".to_owned()),
            Some("cargo test".to_owned()),
            None,
        );

        assert_eq!(step.display_name(), "Run tests");
    }

    #[test]
    fn display_name_returns_id_without_explicit_name() {
        let step = Step::new(
            Some("step-id".to_owned()),
            None,
            Some("cargo test".to_owned()),
            None,
        );

        assert_eq!(step.display_name(), "step-id");
    }

    #[test]
    fn display_name_returns_script_without_name_or_id() {
        let step = Step::for_test(Some("cargo test"), None, None);

        assert_eq!(step.display_name(), "cargo test");
    }

    #[test]
    fn display_name_returns_action_without_name_id_or_script() {
        let step = Step::for_test(None, None, Some("./action"));

        assert_eq!(step.display_name(), "./action");
    }

    #[test]
    fn display_name_returns_fallback_without_displayable_fields() {
        let step = Step::for_test(None, None, None);

        assert_eq!(step.display_name(), "unnamed step");
    }

    #[derive(Default)]
    struct StubClassifier {
        package_registry: bool,
        http_request: bool,
        network_command: bool,
        remote_mutation: bool,
    }

    impl crate::traits::NetworkCommandClassifier for StubClassifier {
        fn accesses_package_registry(&self, _script: &str) -> bool {
            self.package_registry
        }

        fn issues_http_request(&self, _script: &str) -> bool {
            self.http_request
        }

        fn uses_network_command(&self, _script: &str) -> bool {
            self.network_command
        }

        fn mutates_remote_environment(&self, _script: &str) -> bool {
            self.remote_mutation
        }
    }

    #[test]
    fn network_access_reason_returns_none_for_package_registry_access() {
        let step = Step::for_test(Some("echo hi"), None, None);
        let classifier = StubClassifier {
            package_registry: true,
            http_request: true,
            network_command: true,
            ..StubClassifier::default()
        };

        assert_eq!(step.network_access_reason(&classifier), None);
    }

    #[test]
    fn network_access_reason_reports_http_requests() {
        let step = Step::for_test(Some("echo hi"), None, None);
        let classifier = StubClassifier {
            http_request: true,
            network_command: true,
            ..StubClassifier::default()
        };

        assert_eq!(
            step.network_access_reason(&classifier),
            Some("network access is disabled; the step would send an HTTP request")
        );
    }

    #[test]
    fn network_access_reason_reports_network_commands() {
        let step = Step::for_test(Some("echo hi"), None, None);
        let classifier = StubClassifier {
            network_command: true,
            ..StubClassifier::default()
        };

        assert_eq!(
            step.network_access_reason(&classifier),
            Some("network access is disabled; the step would use a network command")
        );
    }

    #[test]
    fn network_access_reason_returns_none_without_network_signal() {
        let step = Step::for_test(Some("echo hi"), None, None);
        let classifier = StubClassifier::default();

        assert_eq!(step.network_access_reason(&classifier), None);
    }

    #[test]
    fn network_access_reason_returns_none_without_script() {
        let step = Step::for_test(None, None, Some("actions/checkout@v4"));
        let classifier = StubClassifier {
            http_request: true,
            ..StubClassifier::default()
        };

        assert_eq!(step.network_access_reason(&classifier), None);
    }

    #[test]
    fn network_policy_violation_reports_remote_mutation() {
        let step = Step::for_test(Some("echo hi"), None, None);
        let classifier = StubClassifier {
            remote_mutation: true,
            ..StubClassifier::default()
        };

        assert_eq!(
            step.network_policy_violation(&classifier),
            Some("network operation blocked; the step would modify a remote environment")
        );
    }

    #[test]
    fn network_policy_violation_returns_none_without_remote_mutation() {
        let step = Step::for_test(Some("echo hi"), None, None);
        let classifier = StubClassifier::default();

        assert_eq!(step.network_policy_violation(&classifier), None);
    }

    #[test]
    fn network_policy_violation_returns_none_without_script() {
        let step = Step::for_test(None, None, Some("actions/checkout@v4"));
        let classifier = StubClassifier {
            remote_mutation: true,
            ..StubClassifier::default()
        };

        assert_eq!(step.network_policy_violation(&classifier), None);
    }

    #[test]
    fn r#if_returns_condition() {
        let step =
            Step::for_test(Some("echo"), None, None).with_if_condition(Some("always()".into()));

        assert_eq!(step.r#if(), Some("always()"));
    }

    #[test]
    fn if_condition_returns_condition() {
        let step =
            Step::for_test(Some("echo"), None, None).with_if_condition(Some("always()".into()));

        assert_eq!(step.if_condition(), Some("always()"));
    }
}
