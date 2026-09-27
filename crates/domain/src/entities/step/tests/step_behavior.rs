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
    #[test]
    fn package_manager_network_access_is_allowed_by_default() {
        for command in [
            "apt-get install curl",
            "dnf install curl",
            "npm install",
            "pip install requests",
            "cargo fetch",
            "go mod download",
        ] {
            let step = Step::for_test(Some(command), None, None);

            assert_eq!(step.network_access_reason(), None);
            assert_eq!(step.network_policy_violation(), None);
        }
    }

    #[test]
    fn remote_mutations_are_blocked_even_when_network_is_enabled() {
        for command in [
            "git push origin main",
            "npm publish",
            "curl --request POST https://example.com",
            "curl --data payload https://example.com",
        ] {
            let step = Step::for_test(Some(command), None, None);

            assert_eq!(
                step.network_policy_violation(),
                Some("network operation blocked; the step would modify a remote environment")
            );
        }
    }
    #[test]
    fn compiler_flags_are_not_remote_mutations() {
        let step = Step::for_test(
            Some("cargo clippy --all-targets --locked -- -D warnings"),
            None,
            None,
        );

        assert_eq!(step.network_policy_violation(), None);
    }
}
