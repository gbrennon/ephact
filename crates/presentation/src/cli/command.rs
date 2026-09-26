use clap::Subcommand;

#[derive(Subcommand)]
pub enum Command {
    Run(Box<super::run_args::RunArgs>),
    ListWorkflows(Box<super::list_workflows_args::ListWorkflowsArgs>),
    ListActions(Box<super::list_actions_args::ListActionsArgs>),
    #[command(subcommand)]
    Settings(super::settings_command::SettingsCommand),
    Tui,
    Cli,
}
