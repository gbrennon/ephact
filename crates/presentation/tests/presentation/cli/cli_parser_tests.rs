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
    fn no_args_selects_tui_by_default() {
        let cli = CliParser::try_parse_from(["ephact"]).expect("no subcommand should parse");

        assert!(!cli.has_explicit_command());
        assert!(matches!(cli.command(), Command::Tui));
    }
}
