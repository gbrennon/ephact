#[cfg(test)]
mod tests {
    use std::time::Duration;

    use ephact::{
        application::dtos::responses::RunSummaryResponse,
        presentation::{
            cli::{CliParser, SettingName, SettingsCommand, command::Command, parse_run_test_args},
            components::terminal::SystemTerminal,
            handlers::RunHandler,
        },
    };

    use crate::common::fakes::{
        fake_list_workflows_port::FakeListWorkflowsPort,
        stub_run_all_workflows_port::StubRunAllWorkflowsPort,
        stub_run_workflow_port::StubRunWorkflowPort,
    };

    fn ok_summary() -> RunSummaryResponse {
        RunSummaryResponse::new("test".to_string(), vec![], true, Duration::ZERO)
    }

    #[test]
    fn root_help_describes_public_commands() {
        let help = match CliParser::try_parse_from(["ephact", "--help"]) {
            Ok(_) => panic!("help should stop argument parsing"),
            Err(error) => error.to_string(),
        };

        for description in [
            "Run workflows locally",
            "List available workflows",
            "List actions",
            "Manage persisted settings",
            "Launch the TUI",
            "Use the CLI interface",
        ] {
            assert!(
                help.contains(description),
                "missing {description:?} in:\n{help}"
            );
        }
    }

    #[test]
    fn settings_help_documents_persistence_and_value_rules() {
        let help = match CliParser::try_parse_from(["ephact", "settings", "--help"]) {
            Ok(_) => panic!("help should stop argument parsing"),
            Err(error) => error.to_string(),
        };

        for guidance in [
            "show reads and displays the current persisted settings",
            "set <name> <value> changes exactly one setting",
            "preserves the other settings",
            "Boolean settings accept only true or false",
            "default-interface accepts tui or cli",
            "navigation-mode accepts standard, vim, or emacs",
            "failure-log-retention-hours requires a positive integer greater than zero",
            "marker accepts one of the existing preset marker strings or arbitrary custom text",
            "forward-ssh is persisted in the host settings store",
            "reset restores all persisted settings to safe defaults",
        ] {
            assert!(help.contains(guidance), "missing {guidance:?} in:\n{help}");
        }
    }

    #[test]
    fn run_help_documents_precedence_network_gate_and_examples() {
        let help = match CliParser::try_parse_from(["ephact", "run", "--help"]) {
            Ok(_) => panic!("help should stop argument parsing"),
            Err(error) => error.to_string(),
        };

        for guidance in [
            "Persisted boolean settings are applied as defaults with logical-OR behavior",
            "cannot turn off a persisted true value by omitting the flag",
            "--failure-log-retention-hours HOURS applies for that run",
            "--forward-ssh requires --allow-network",
            "ephact settings set navigation-mode vim",
            "ephact settings set allow-network true",
            "ephact run --workflow ci.yml --allow-network --forward-ssh",
            "ephact run --failure-log-retention-hours 24",
        ] {
            assert!(help.contains(guidance), "missing {guidance:?} in:\n{help}");
        }
    }

    #[tokio::test]
    async fn run_dispatches_success_without_exiting() {
        let wf_port = StubRunWorkflowPort {
            result: Ok(ok_summary()),
        };
        let all_wf_port = StubRunAllWorkflowsPort {
            result: Ok(ok_summary()),
        };
        let args = parse_run_test_args(&[]);
        let terminal = SystemTerminal;
        let list_port = FakeListWorkflowsPort::new();
        RunHandler::handle_cli(args, &wf_port, &all_wf_port, &list_port, &terminal)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn run_dispatches_with_workflow_flag() {
        let wf_port = StubRunWorkflowPort {
            result: Ok(ok_summary()),
        };
        let all_wf_port = StubRunAllWorkflowsPort {
            result: Ok(ok_summary()),
        };
        let args = parse_run_test_args(&["--workflow", "ci.yml"]);
        let terminal = SystemTerminal;
        let list_port = FakeListWorkflowsPort::new();
        RunHandler::handle_cli(args, &wf_port, &all_wf_port, &list_port, &terminal)
            .await
            .unwrap();
    }

    #[test]
    fn parses_tui_command() {
        let cli = CliParser::try_parse_from(["ephact", "tui"]).expect("tui should parse");
        assert!(matches!(cli.command(), Command::Tui));
    }

    #[test]
    fn parses_cli_command() {
        let cli = CliParser::try_parse_from(["ephact", "cli"]).expect("cli should parse");

        assert!(matches!(cli.command(), Command::Cli));
    }
    #[test]
    fn parses_settings_show_and_set_commands() {
        let show = CliParser::try_parse_from(["ephact", "settings", "show"])
            .expect("settings show should parse");
        assert!(matches!(
            show.command(),
            Command::Settings(SettingsCommand::Show)
        ));

        let set =
            CliParser::try_parse_from(["ephact", "settings", "set", "default-interface", "cli"])
                .expect("settings set should parse");
        match set.command() {
            Command::Settings(SettingsCommand::Set(arguments)) => {
                assert_eq!(arguments.name(), SettingName::DefaultInterface);
                assert_eq!(arguments.value(), "cli");
            }
            _ => panic!("expected settings set"),
        }
    }

    #[test]
    fn parses_failure_log_retention_setting_name() {
        let cli = CliParser::try_parse_from([
            "ephact",
            "settings",
            "set",
            "failure-log-retention-hours",
            "72",
        ])
        .expect("retention setting should parse");

        match cli.command() {
            Command::Settings(SettingsCommand::Set(arguments)) => {
                assert_eq!(arguments.name(), SettingName::FailureLogRetentionHours);
                assert_eq!(arguments.value(), "72");
            }
            _ => panic!("expected settings set"),
        }
    }

    #[test]
    fn parses_forward_ssh_setting_name() {
        let cli = CliParser::try_parse_from(["ephact", "settings", "set", "forward-ssh", "true"])
            .expect("forward-ssh setting should parse");

        match cli.command() {
            Command::Settings(SettingsCommand::Set(arguments)) => {
                assert_eq!(arguments.name(), SettingName::ForwardSsh);
                assert_eq!(arguments.value(), "true");
            }
            _ => panic!("expected settings set"),
        }
    }

    #[test]
    fn parses_marker_setting_name() {
        let cli = CliParser::try_parse_from(["ephact", "settings", "set", "marker", "❯"])
            .expect("marker setting should parse");

        match cli.command() {
            Command::Settings(SettingsCommand::Set(arguments)) => {
                assert_eq!(arguments.name(), SettingName::Marker);
                assert_eq!(arguments.value(), "❯");
            }
            _ => panic!("expected settings set"),
        }
    }
    #[test]
    fn rejects_zero_failure_log_retention_hours_before_dispatch() {
        let error = match CliParser::try_parse_from([
            "ephact",
            "run",
            "--failure-log-retention-hours",
            "0",
        ]) {
            Ok(_) => panic!("zero retention should fail during argument parsing"),
            Err(error) => error,
        };

        assert!(error.to_string().contains("greater than zero"));
    }

    #[test]
    fn rejects_non_numeric_failure_log_retention_hours_before_dispatch() {
        let error = match CliParser::try_parse_from([
            "ephact",
            "run",
            "--failure-log-retention-hours",
            "invalid",
        ]) {
            Ok(_) => panic!("non-numeric retention should fail during argument parsing"),
            Err(error) => error,
        };

        assert!(error.to_string().contains("positive integer"));
    }

    #[test]
    fn no_args_selects_tui_by_default() {
        let cli = CliParser::try_parse_from(["ephact"]).expect("no subcommand should parse");

        assert!(!cli.has_explicit_command());
        assert!(matches!(cli.command(), Command::Tui));
    }

    #[test]
    fn help_describes_woodpecker_support() {
        let command = CliParser::build_command();

        assert!(
            command
                .get_about()
                .is_some_and(|about| about.to_string().contains("Woodpecker"))
        );
        assert!(command.get_long_about().is_some_and(|long_about| {
            let long_about = long_about.to_string();
            long_about.contains("Woodpecker") && long_about.contains(".woodpecker")
        }));
    }
}
