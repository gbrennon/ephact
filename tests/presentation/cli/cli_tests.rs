#[cfg(test)]
mod tests {
    use std::error::Error;

    use ephact::{
        application::{
            dtos::ShowProjectBrandingInfoResponse, ports::inbound::ShowProjectBrandingInfoPort,
        },
        presentation::{cli::Cli, components::terminal::Terminal},
    };

    use crate::common::fakes::{
        fake_list_actions_port::FakeListActionsPort,
        fake_list_workflows_port::FakeListWorkflowsPort,
        fake_run_all_workflows_port::FakeRunAllWorkflowsPort,
        fake_run_workflow_port::FakeRunWorkflowPort,
    };
    use crate::fakes::FakeDiscoverRunInputsPort;

    struct FakeShowProjectBrandingInfoPort;

    impl ShowProjectBrandingInfoPort for FakeShowProjectBrandingInfoPort {
        fn execute(&self) -> Result<ShowProjectBrandingInfoResponse, Box<dyn Error>> {
            Ok(ShowProjectBrandingInfoResponse::new(
                "ephact".to_string(),
                "Ephemeral action runner".to_string(),
                "0.1.0".to_string(),
                "shield".to_string(),
            ))
        }
    }
    struct TestTerminal;

    impl Terminal for TestTerminal {
        fn dimensions(&self) -> (usize, usize) {
            (100, 40)
        }

        fn write_text(&self, _text: &str) -> std::io::Result<()> {
            Ok(())
        }

        fn read_line(&self) -> std::io::Result<String> {
            Ok(String::new())
        }
    }

    fn make_cli() -> Cli {
        Cli::new(
            Box::new(FakeRunWorkflowPort::new(true)),
            Box::new(FakeRunAllWorkflowsPort::new(true)),
            Box::new(FakeDiscoverRunInputsPort::new()),
            Box::new(FakeListWorkflowsPort::new()),
            Box::new(FakeListActionsPort::new()),
            Box::new(FakeShowProjectBrandingInfoPort),
        )
    }

    #[test]
    fn new_creates_cli_instance() {
        let _cli = make_cli();
    }
    #[test]
    fn run_no_args_displays_help() {
        let cli = make_cli();
        let result = cli.run(["ephact"]);
        assert!(result.is_ok());
    }

    #[test]
    fn run_no_args_displays_supported_platforms() {
        let cli = make_cli();
        let output = cli.run_with_terminal(["ephact"], &TestTerminal).unwrap();
        assert!(output.contains("Forgejo"));
        assert!(output.contains("GitHub"));
    }

    #[test]
    fn run_run_subcommand_succeeds() {
        let cli = make_cli();
        let result = cli.run(["ephact", "run"]);
        assert!(result.is_ok());
    }

    #[test]
    fn run_run_subcommand_propagates_workflow_failure() {
        let cli = Cli::new(
            Box::new(FakeRunWorkflowPort::new(false)),
            Box::new(FakeRunAllWorkflowsPort::new(false)),
            Box::new(FakeDiscoverRunInputsPort::new()),
            Box::new(FakeListWorkflowsPort::new()),
            Box::new(FakeListActionsPort::new()),
            Box::new(FakeShowProjectBrandingInfoPort),
        );
        let result = cli.run(["ephact", "run"]);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("workflow failed"));
    }

    #[test]
    fn run_invalid_subcommand_returns_error() {
        let cli = make_cli();
        let result = cli.run(["ephact", "nonexistent"]);
        assert!(result.is_err());
    }

    #[test]
    fn run_invalid_flag_returns_error() {
        let cli = make_cli();
        let result = cli.run(["ephact", "--nonexistent-flag"]);
        assert!(result.is_err());
    }

    #[test]
    fn run_explicit_help_flag_succeeds() {
        let cli = make_cli();
        let result = cli.run(["ephact", "--help"]);
        assert!(result.is_ok());
    }

    #[test]
    fn run_explicit_help_flag_displays_supported_platforms_and_workflows() {
        let cli = make_cli();
        let output = cli
            .run_with_terminal(["ephact", "--help"], &TestTerminal)
            .unwrap();
        assert!(output.contains("Forgejo"));
        assert!(output.contains("GitHub"));
        assert!(output.contains(".forgejo/workflows"));
        assert!(output.contains(".github/workflows"));
    }

    #[test]
    fn run_subcommand_help_flag_succeeds() {
        let cli = make_cli();
        let result = cli.run(["ephact", "run", "--help"]);
        assert!(result.is_ok());
    }
}
