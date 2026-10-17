use std::ffi::OsString;

use clap::Parser;

const SUPPORTED_PLATFORMS: &str = "Forgejo, GitHub, Woodpecker";
const SUPPORTED_WORKFLOWS: &str = ".forgejo/workflows, .github/workflows, .woodpecker";

#[derive(Parser)]
#[command(
    name = "ephact",
    after_long_help = r#"EXAMPLES:
    ephact run --event pull_request
    ephact run --event pull_request --workflow CI --job test
    ephact run --event pull_request --secret TOKEN=abc123

Noninteractive runs require `--event`; interactive mode selects a `pull_request`
workflow. The CI host is auto-detected from the repository layout and manages
ephemeral copies internally."#
)]
pub struct CliParser {
    #[command(subcommand)]
    command: Option<super::command::Command>,
}

impl CliParser {
    pub fn command(self) -> super::command::Command {
        self.command.unwrap_or(super::command::Command::Tui)
    }

    pub fn has_explicit_command(&self) -> bool {
        self.command.is_some()
    }

    /// Parses application arguments after removing the optional `cli` alias.
    pub fn try_parse_invocation_from(args: &[OsString]) -> Result<Self, clap::Error> {
        let mut normalized_args = args.to_vec();
        if normalized_args
            .get(1)
            .is_some_and(|argument| argument == "cli")
        {
            normalized_args.remove(1);
        }
        Self::try_parse_from(normalized_args)
    }

    /// Reports whether the parsed invocation selects the `run` command.
    pub fn is_run_command(&self) -> bool {
        matches!(self.command.as_ref(), Some(super::command::Command::Run(_)))
    }

    /// Reports whether the parsed invocation selects a workflow or action listing.
    pub fn is_discovery_command(&self) -> bool {
        matches!(
            self.command.as_ref(),
            Some(super::command::Command::ListWorkflows(_))
                | Some(super::command::Command::ListActions(_))
        )
    }

    /// Reports whether the parsed invocation explicitly selects the `tui` command.
    pub fn is_tui_command(&self) -> bool {
        matches!(self.command.as_ref(), Some(super::command::Command::Tui))
    }

    /// Reports whether the parsed run command requests verbose output.
    pub fn is_verbose_requested(&self) -> bool {
        self.run_arguments()
            .map(super::run_args::RunArgs::invocation_flags)
            .is_some_and(|flags| flags.is_verbose_requested())
    }

    /// Reports whether the parsed run command allows network access.
    pub fn is_network_allowed(&self) -> bool {
        self.run_arguments()
            .map(super::run_args::RunArgs::invocation_flags)
            .is_some_and(|flags| flags.is_network_allowed())
    }

    /// Reports whether the parsed run command requests SSH forwarding.
    pub fn is_ssh_forwarding_requested(&self) -> bool {
        self.run_arguments()
            .map(super::run_args::RunArgs::invocation_flags)
            .is_some_and(|flags| flags.is_ssh_forwarding_requested())
    }

    fn run_arguments(&self) -> Option<&super::run_args::RunArgs> {
        match self.command.as_ref() {
            Some(super::command::Command::Run(arguments)) => Some(arguments),
            _ => None,
        }
    }
    /// Builds the base CLI command with dynamic platform and workflow descriptions.
    pub fn build_command() -> clap::Command {
        let platforms = SUPPORTED_PLATFORMS;
        let workflows = SUPPORTED_WORKFLOWS;
        <Self as clap::CommandFactory>::command()
            .about(format!(
                "Run CI workflows locally in ephemeral repositories ({platforms})"
            ))
            .long_about(format!(
                "Runs CI workflows in an ephemeral copy of a repository. \
                 The CI host is auto-detected from the repository layout; \
                 see `run --help` for the available options.\n\n\
                 Supported platforms: {platforms}\n\
                 Supported workflow directories: {workflows}"
            ))
    }

    /// Attempts to parse command-line arguments, returning a [`clap::Error`] on failure.
    pub fn try_parse_from<I, T>(args: I) -> Result<Self, clap::Error>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        let matches = Self::build_command().try_get_matches_from(args)?;
        <Self as clap::FromArgMatches>::from_arg_matches(&matches)
    }

    /// Parses command-line arguments, printing an error and exiting on failure.
    pub fn parse_from<I, T>(args: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        Self::try_parse_from(args).unwrap_or_else(|error| error.exit())
    }
}

/// Parses CLI arguments for the `run` subcommand from a string slice.
///
/// Intended for use in tests - avoids depending on `std::env::args()`.
pub fn parse_run_test_args(args: &[&str]) -> super::run_args::RunArgs {
    let mut full: Vec<&str> = vec!["ephact", "run"];
    full.extend_from_slice(args);
    let cli = CliParser::parse_from(&full);
    match cli.command() {
        super::command::Command::Run(args) => *args,
        _ => unreachable!(),
    }
}

/// Parses CLI arguments for the `list-workflows` subcommand from a string slice.
///
/// Intended for use in tests.
pub fn parse_list_workflows_test_args(
    args: &[&str],
) -> super::list_workflows_args::ListWorkflowsArgs {
    let mut full: Vec<&str> = vec!["ephact", "list-workflows"];
    full.extend_from_slice(args);
    let cli = CliParser::parse_from(&full);
    match cli.command() {
        super::command::Command::ListWorkflows(args) => *args,
        _ => unreachable!(),
    }
}

/// Parses CLI arguments for the `list-actions` subcommand from a string slice.
///
/// Intended for use in tests.
pub fn parse_list_actions_test_args(args: &[&str]) -> super::list_actions_args::ListActionsArgs {
    let mut full: Vec<&str> = vec!["ephact", "list-actions"];
    full.extend_from_slice(args);
    let cli = CliParser::parse_from(&full);
    match cli.command() {
        super::command::Command::ListActions(args) => *args,
        _ => unreachable!(),
    }
}
#[cfg(test)]
mod tests {
    use std::ffi::OsString;

    use super::CliParser;

    #[test]
    fn parses_typed_invocation_flags_for_run_command() {
        let args = vec![
            OsString::from("ephact"),
            OsString::from("run"),
            OsString::from("--verbose"),
            OsString::from("--allow-network"),
            OsString::from("--forward-ssh"),
        ];

        let parser =
            CliParser::try_parse_invocation_from(&args).expect("run arguments should parse");

        assert!(parser.is_run_command());
        assert!(parser.is_verbose_requested());
        assert!(parser.is_network_allowed());
        assert!(parser.is_ssh_forwarding_requested());
        assert!(!parser.is_tui_command());
    }

    #[test]
    fn parses_typed_invocation_flags_for_cli_alias() {
        let args = vec![
            OsString::from("ephact"),
            OsString::from("cli"),
            OsString::from("tui"),
        ];

        let parser =
            CliParser::try_parse_invocation_from(&args).expect("cli alias arguments should parse");

        assert!(!parser.is_run_command());
        assert!(parser.is_tui_command());
        assert!(!parser.is_network_allowed());
        assert!(!parser.is_ssh_forwarding_requested());
    }
}
