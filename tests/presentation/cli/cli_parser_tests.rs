#[cfg(test)]
mod tests {
    use ephact::presentation::cli::{CliParser, command::Command, parse_run_test_args};
    use ephact::presentation::components::terminal::SystemTerminal;
    use ephact::presentation::handlers::RunHandler;

    use std::time::Duration;

    use ephact::application::dtos::responses::RunSummaryResponse;

    use crate::common::fakes::{
        fake_list_workflows_port::FakeListWorkflowsPort,
        stub_run_all_workflows_port::StubRunAllWorkflowsPort,
        stub_run_workflow_port::StubRunWorkflowPort,
    };

    fn ok_summary() -> RunSummaryResponse {
        RunSummaryResponse::new("test".to_string(), vec![], true, Duration::ZERO)
    }

    #[test]
    fn run_dispatches_success_without_exiting() {
        let wf_port = StubRunWorkflowPort {
            result: Ok(ok_summary()),
        };
        let all_wf_port = StubRunAllWorkflowsPort {
            result: Ok(ok_summary()),
        };
        let args = parse_run_test_args(&[]);
        let terminal = SystemTerminal;
        let list_port = FakeListWorkflowsPort::new();
        RunHandler::handle_cli(args, &wf_port, &all_wf_port, &list_port, &terminal).unwrap();
    }

    #[test]
    fn run_dispatches_with_workflow_flag() {
        let wf_port = StubRunWorkflowPort {
            result: Ok(ok_summary()),
        };
        let all_wf_port = StubRunAllWorkflowsPort {
            result: Ok(ok_summary()),
        };
        let args = parse_run_test_args(&["--workflow", "ci.yml"]);
        let terminal = SystemTerminal;
        let list_port = FakeListWorkflowsPort::new();
        RunHandler::handle_cli(args, &wf_port, &all_wf_port, &list_port, &terminal).unwrap();
    }

    #[test]
    fn parses_tui_command() {
        let cli = CliParser::try_parse_from(["ephact", "tui"]).expect("tui should parse");
        assert!(matches!(cli.command(), Command::Tui));
    }

    #[test]
    fn no_args_displays_help() {
        let result = CliParser::try_parse_from(["ephact"]);
        let err = match result {
            Ok(_) => panic!("expected missing-command error"),
            Err(e) => e,
        };
        assert_eq!(
            err.kind(),
            clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        );
    }
}
