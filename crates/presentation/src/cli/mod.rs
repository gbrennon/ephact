#[allow(clippy::module_inception)]
pub mod cli;
pub mod cli_dependencies;
pub mod cli_parser;
pub mod command;
mod input_collector;
mod input_declaration_context;
mod input_parser;
pub mod list_actions_args;
pub mod list_workflows_args;
pub mod run_args;
pub mod run_invocation_flags;
pub mod run_progress_handler;
pub mod settings_set_args;

pub use cli::Cli;
pub use cli_dependencies::CliDependencies;
pub use cli_parser::{
    CliParser, parse_list_actions_test_args, parse_list_workflows_test_args, parse_run_test_args,
};
pub use input_collector::InputCollector;
pub use input_parser::InputParser;
pub use list_actions_args::ListActionsArgs;
pub use list_workflows_args::ListWorkflowsArgs;
pub use run_args::RunArgs;
pub use run_invocation_flags::RunInvocationFlags;
pub use run_progress_handler::{RunProgressHandler, TuiProgressStream};
pub use settings_set_args::{SettingName, SettingsCommand, SettingsSetArgs};

pub use crate::handlers::{
    DiagnosticStores, ListActionsHandler, ListWorkflowsHandler, PreflightPorts, RunHandler,
};
