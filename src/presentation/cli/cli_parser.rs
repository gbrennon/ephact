use clap::Parser;

const SUPPORTED_PLATFORMS: &str = "Forgejo, GitHub";
const SUPPORTED_WORKFLOWS: &str = ".forgejo/workflows, .github/workflows";

#[derive(Parser)]
#[command(
    name = "ephact",
    arg_required_else_help = true,
    after_long_help = r#"EXAMPLES:
    ephact run
    ephact run --workflow ci.yml --job test
    ephact run --event push --secret TOKEN=abc123
    ephact run --container-engine docker

CI host from the repository layout and manages ephemeral copies internally."#
)]
pub struct CliParser {
    #[command(subcommand)]
    command: super::command::Command,
}

impl CliParser {
    pub fn command(self) -> super::command::Command {
        self.command
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
                "Runs CI workflows in an ephemeral copy of a repository using `act`. \
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
