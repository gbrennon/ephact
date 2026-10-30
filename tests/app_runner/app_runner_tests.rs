use std::ffi::OsString;

use ephact::{app_runner::ApplicationRunner, presentation::cli::CliParser};

enum TestSupport {}

impl TestSupport {
    fn parser(values: &[&str]) -> CliParser {
        let arguments = values.iter().map(OsString::from).collect::<Vec<_>>();
        CliParser::try_parse_invocation_from(&arguments).expect("arguments should parse")
    }
}

mod tests {
    use super::*;
    #[test]
    fn persisted_forwarding_is_disabled_without_network_access() {
        let invocation = TestSupport::parser(&["ephact", "run"]);
        let config = ApplicationRunner::resolve_ssh_forwarding(
            true,
            invocation.is_network_allowed(),
            invocation.is_run_command(),
            invocation.is_ssh_forwarding_requested(),
        );

        assert!(!config.is_enabled());
    }

    #[test]
    fn explicit_run_forwarding_is_enabled_with_network_access() {
        let invocation =
            TestSupport::parser(&["ephact", "run", "--allow-network", "--forward-ssh"]);
        let config = ApplicationRunner::resolve_ssh_forwarding(
            false,
            invocation.is_network_allowed(),
            invocation.is_run_command(),
            invocation.is_ssh_forwarding_requested(),
        );

        assert!(config.is_enabled());
    }

    #[test]
    fn persisted_forwarding_is_enabled_for_tui_commands() {
        let invocation = TestSupport::parser(&["ephact", "tui"]);
        let config = ApplicationRunner::resolve_ssh_forwarding(
            true,
            true,
            invocation.is_tui_command(),
            invocation.is_ssh_forwarding_requested(),
        );

        assert!(config.is_enabled());
    }

    #[test]
    fn persisted_forwarding_is_enabled_for_cli_tui_commands() {
        let invocation = TestSupport::parser(&["ephact", "cli", "tui"]);
        let config = ApplicationRunner::resolve_ssh_forwarding(
            true,
            true,
            invocation.is_tui_command(),
            invocation.is_ssh_forwarding_requested(),
        );

        assert!(config.is_enabled());
    }

    #[test]
    fn persisted_forwarding_is_enabled_for_default_tui() {
        let config = ApplicationRunner::resolve_ssh_forwarding(true, true, true, false);

        assert!(config.is_enabled());
    }

    #[test]
    fn persisted_forwarding_is_disabled_for_unrelated_commands() {
        let invocation = TestSupport::parser(&["ephact", "settings", "show"]);
        let config = ApplicationRunner::resolve_ssh_forwarding(
            true,
            true,
            invocation.is_run_command() || invocation.is_tui_command(),
            invocation.is_ssh_forwarding_requested(),
        );

        assert!(!config.is_enabled());
    }
}
