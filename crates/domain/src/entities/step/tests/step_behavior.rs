#[cfg(test)]
mod tests {
    use super::super::super::*;

    impl Step {
        fn for_test(run: Option<&str>, shell: Option<&str>, uses: Option<&str>) -> Self {
            Self::new(None, None, run.map(str::to_owned), uses.map(str::to_owned))
                .with_shell(shell.map(str::to_owned))
        }
    }

    #[test]
    fn a_step_with_a_script_is_a_run_step() {
        let step = Step::for_test(Some("cargo test"), None, None);

        assert!(step.is_run_step());
        assert!(!step.is_uses_step());
        assert_eq!(step.step_type(), StepType::Run);
    }

    #[test]
    fn a_step_with_an_action_reference_is_a_uses_step() {
        let step = Step::for_test(None, None, Some("actions/checkout@v4"));

        assert!(step.is_uses_step());
        assert!(!step.is_run_step());
        assert_eq!(step.step_type(), StepType::Uses);
    }

    #[test]
    fn a_local_action_reference_is_a_composite_step() {
        assert_eq!(
            Step::for_test(None, None, Some("./action")).step_type(),
            StepType::Composite
        );
    }

    #[test]
    fn a_step_without_run_or_uses_is_invalid() {
        assert_eq!(
            Step::for_test(None, None, None).step_type(),
            StepType::Invalid
        );
    }

    #[test]
    fn effective_shell_uses_step_shell_when_set() {
        assert_eq!(
            Step::for_test(Some("echo hello"), Some("pwsh"), None).effective_shell("bash"),
            "pwsh"
        );
    }

    #[test]
    fn continues_on_error_matches_truthy_scalars_case_insensitively() {
        let with_flag = |flag: &str| {
            Step::new(None, None, Some("echo".to_owned()), None)
                .with_continue_on_error(Some(flag.to_owned()))
        };

        assert!(with_flag("true").continues_on_error());
        assert!(with_flag("True").continues_on_error());
        assert!(!with_flag("false").continues_on_error());
        assert!(!Step::for_test(Some("echo"), None, None).continues_on_error());
    }

    #[test]
    fn display_name_falls_back_through_id_script_and_action() {
        let named = Step::new(
            Some("step-id".to_owned()),
            Some("Run tests".to_owned()),
            Some("cargo test".to_owned()),
            None,
        );
        let identified = Step::new(
            Some("step-id".to_owned()),
            None,
            Some("cargo test".to_owned()),
            None,
        );

        assert_eq!(named.display_name(), "Run tests");
        assert_eq!(identified.display_name(), "step-id");
        assert_eq!(
            Step::for_test(Some("cargo test"), None, None).display_name(),
            "cargo test"
        );
        assert_eq!(
            Step::for_test(None, None, Some("./action")).display_name(),
            "./action"
        );
        assert_eq!(
            Step::for_test(None, None, None).display_name(),
            "unnamed step"
        );
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
    fn package_manager_access_is_allowed_over_other_signals() {
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
    fn network_policy_violation_delegates_remote_mutation_message() {
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
    fn absence_of_remote_mutation_is_no_violation() {
        let step = Step::for_test(Some("echo hi"), None, None);
        let classifier = StubClassifier::default();

        assert_eq!(step.network_policy_violation(&classifier), None);
    }

    #[test]
    fn if_condition_accessors_expose_the_condition() {
        let step =
            Step::for_test(Some("echo"), None, None).with_if_condition(Some("always()".into()));

        assert_eq!(step.r#if(), Some("always()"));
        assert_eq!(step.if_condition(), Some("always()"));
    }

    #[test]
    fn http_requests_report_disabled_network_access() {
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
    fn network_commands_report_disabled_network_access() {
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
    fn network_access_reason_without_any_signal_is_none() {
        let step = Step::for_test(Some("echo hi"), None, None);
        let classifier = StubClassifier::default();

        assert_eq!(step.network_access_reason(&classifier), None);
    }

    #[test]
    fn network_methods_are_none_without_a_script() {
        let step = Step::for_test(None, None, Some("actions/checkout@v4"));
        let classifier = StubClassifier {
            http_request: true,
            remote_mutation: true,
            ..StubClassifier::default()
        };

        assert_eq!(step.network_access_reason(&classifier), None);
        assert_eq!(step.network_policy_violation(&classifier), None);
    }
}
