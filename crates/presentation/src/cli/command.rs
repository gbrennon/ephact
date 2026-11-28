use clap::Subcommand;

#[derive(Subcommand)]
#[command(
    about = "Select an operation for this repository.",
    long_about = "Select an operation for running workflows, discovering workflows and actions, \
                  managing persisted settings, or choosing an interface."
)]
pub enum Command {
    #[command(about = "Run workflows locally.")]
    Run(Box<super::run_args::RunArgs>),
    #[command(about = "List available workflows.")]
    ListWorkflows(Box<super::list_workflows_args::ListWorkflowsArgs>),
    #[command(about = "List actions in supported repositories.")]
    ListActions(Box<super::list_actions_args::ListActionsArgs>),
    #[command(about = "Manage persisted settings.")]
    #[command(subcommand)]
    Settings(super::settings_set_args::SettingsCommand),
    #[command(about = "Launch the TUI.")]
    Tui,
    #[command(about = "Use the CLI interface.")]
    Cli,
}
