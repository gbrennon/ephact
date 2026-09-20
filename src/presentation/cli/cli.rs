use std::ffi::OsString;
use std::sync::Arc;

use super::super::components::{
    banner::Banner,
    box_component::BoxComponent,
    content::ContentComponent,
    terminal::{SystemTerminal, Terminal},
};
use super::run_progress_handler::TuiProgressStream;
use super::{cli_parser::CliParser, command::Command};
use crate::application::ports::inbound::{
    list_actions_port::ListActionsPort, list_workflows_port::ListWorkflowsPort,
    run_all_workflows_port::RunAllWorkflowsPort, run_workflow_port::RunWorkflowPort,
    show_project_branding_info_port::ShowProjectBrandingInfoPort,
};
use crate::application::ports::outbound::DiscoverRunInputsPort;
use crate::presentation::handlers::{
    DiagnosticStores, ListActionsHandler, ListWorkflowsHandler, PreflightPorts, RunHandler,
};
use crate::presentation::tui::TuiRunner;

pub struct Cli {
    run_workflow_port: Arc<dyn RunWorkflowPort>,
    run_all_workflows_port: Box<dyn RunAllWorkflowsPort>,
    discover_run_inputs_port: Arc<dyn DiscoverRunInputsPort>,
    list_workflows_port: Arc<dyn ListWorkflowsPort>,
    list_actions_port: Arc<dyn ListActionsPort>,
    show_project_branding_info_port: Box<dyn ShowProjectBrandingInfoPort>,
    failure_log_error_store: crate::infrastructure::logging::FailureLogErrorStore,
    failure_log_path_store: crate::infrastructure::logging::FailureLogPathStore,
    tui_runner: TuiRunner,
}
pub use super::cli_dependencies::{
    CliDependencies, CliListDependencies, CliParts, CliRunDependencies,
};

impl Cli {
    pub fn new(dependencies: CliDependencies) -> Self {
        Self::new_with_failure_stores(
            dependencies,
            crate::infrastructure::logging::FailureLogStores::new(),
        )
    }

    pub fn new_with_failure_stores(
        dependencies: CliDependencies,
        failure_log_stores: crate::infrastructure::logging::FailureLogStores,
    ) -> Self {
        let (
            run_workflow_port,
            run_all_workflows_port,
            discover_run_inputs_port,
            list_workflows_port,
            list_actions_port,
            show_project_branding_info_port,
        ) = dependencies.into_parts();
        let run_workflow_port: Arc<dyn RunWorkflowPort> = Arc::from(run_workflow_port);
        let discover_run_inputs_port: Arc<dyn DiscoverRunInputsPort> =
            Arc::from(discover_run_inputs_port);
        let list_workflows_port: Arc<dyn ListWorkflowsPort> = Arc::from(list_workflows_port);
        let list_actions_port: Arc<dyn ListActionsPort> = Arc::from(list_actions_port);
        let tui_runner = TuiRunner::new(
            list_workflows_port.clone(),
            list_actions_port.clone(),
            run_workflow_port.clone(),
        )
        .with_input_discovery(discover_run_inputs_port.clone());
        Self {
            run_workflow_port,
            run_all_workflows_port,
            discover_run_inputs_port,
            list_workflows_port,
            list_actions_port,
            show_project_branding_info_port,
            failure_log_error_store: failure_log_stores.error_store(),
            failure_log_path_store: failure_log_stores.path_store(),
            tui_runner,
        }
    }

    pub fn new_with_failure_stores_and_progress_stream(
        dependencies: CliDependencies,
        failure_log_stores: crate::infrastructure::logging::FailureLogStores,
        progress_stream: TuiProgressStream,
    ) -> Self {
        let mut cli = Self::new_with_failure_stores(dependencies, failure_log_stores);
        cli.tui_runner = cli.tui_runner.with_progress_stream(progress_stream);
        cli
    }

    pub fn new_with_failure_stores_and_tui(
        dependencies: CliDependencies,
        failure_log_stores: crate::infrastructure::logging::FailureLogStores,
        tui_runner: TuiRunner,
    ) -> Self {
        let (
            run_workflow_port,
            run_all_workflows_port,
            discover_run_inputs_port,
            list_workflows_port,
            list_actions_port,
            show_project_branding_info_port,
        ) = dependencies.into_parts();
        let run_workflow_port: Arc<dyn RunWorkflowPort> = Arc::from(run_workflow_port);
        let discover_run_inputs_port: Arc<dyn DiscoverRunInputsPort> =
            Arc::from(discover_run_inputs_port);
        let list_workflows_port: Arc<dyn ListWorkflowsPort> = Arc::from(list_workflows_port);
        let list_actions_port: Arc<dyn ListActionsPort> = Arc::from(list_actions_port);
        Self {
            run_workflow_port,
            run_all_workflows_port,
            discover_run_inputs_port,
            list_workflows_port,
            list_actions_port,
            show_project_branding_info_port,
            failure_log_error_store: failure_log_stores.error_store(),
            failure_log_path_store: failure_log_stores.path_store(),
            tui_runner,
        }
    }
}

impl Cli {
    pub fn run<I, T>(self, args: I) -> Result<(), Box<dyn std::error::Error>>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        tokio::runtime::Runtime::new()?.block_on(self.run_async(args))
    }

    async fn run_async<I, T>(self, args: I) -> Result<(), Box<dyn std::error::Error>>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let terminal = SystemTerminal;
        let output = self.run_with_terminal_async(args, &terminal).await?;
        print!("{output}");
        Ok(())
    }
    pub fn run_with_terminal<I, T>(
        self,
        args: I,
        terminal: &dyn Terminal,
    ) -> Result<String, Box<dyn std::error::Error>>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        tokio::runtime::Runtime::new()?.block_on(self.run_with_terminal_async(args, terminal))
    }

    async fn run_with_terminal_async<I, T>(
        self,
        args: I,
        terminal: &dyn Terminal,
    ) -> Result<String, Box<dyn std::error::Error>>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let args: Vec<OsString> = args.into_iter().map(Into::into).collect();
        if Self::is_tui_command(&args) {
            self.tui_runner.run().await?;
            return Ok(String::new());
        }
        self.execute_cli(args, terminal).await
    }

    async fn execute_cli(
        &self,
        args: Vec<OsString>,
        terminal: &dyn Terminal,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let branding = self.show_project_branding_info_port.execute()?;
        let parsed = CliParser::try_parse_from(args);
        let cli = match parsed {
            Ok(cli) => cli,
            Err(e) => return Self::render_parse_error(e),
        };
        let mut output = BoxComponent::new(Banner::new(&branding), terminal).render();
        let command = cli.command();
        self.execute_command(command, terminal, &mut output).await?;
        Ok(output)
    }
    fn is_tui_command(args: &[OsString]) -> bool {
        args.get(1).is_some_and(|arg| arg == "tui")
    }

    fn render_parse_error(e: clap::error::Error) -> Result<String, Box<dyn std::error::Error>> {
        if e.kind() == clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
            || !e.use_stderr()
        {
            Ok(e.to_string())
        } else {
            Err(e.to_string().into())
        }
    }
    async fn execute_command(
        &self,
        command: Command,
        terminal: &dyn Terminal,
        output: &mut String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match command {
            Command::Run(args) => self.execute_run(*args, terminal, output).await,
            Command::ListWorkflows(args) => self.execute_list_workflows(*args, terminal, output),
            Command::ListActions(args) => self.execute_list_actions(*args, terminal, output),
            Command::Tui => self.tui_runner.run().await,
        }
    }

    async fn execute_run(
        &self,
        args: super::run_args::RunArgs,
        terminal: &dyn Terminal,
        output: &mut String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if args.interactive() {
            print!("{output}");
            output.clear();
        }
        let (summary, success) = RunHandler::handle_with_preflight_output_and_diagnostics(
            args,
            &*self.run_workflow_port,
            &*self.run_all_workflows_port,
            PreflightPorts::new(
                &*self.discover_run_inputs_port,
                &*self.list_workflows_port,
                terminal,
            ),
            DiagnosticStores::new(&self.failure_log_error_store, &self.failure_log_path_store),
        )
        .await?;
        output.push_str(&summary);
        if !success {
            print!("{output}");
            return Err("workflow failed; see the run summary for failed steps".into());
        }
        Ok(())
    }

    fn execute_list_workflows(
        &self,
        args: super::list_workflows_args::ListWorkflowsArgs,
        terminal: &dyn Terminal,
        output: &mut String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response =
            ListWorkflowsHandler::handle(&*self.list_workflows_port, args.path().to_path_buf())?;
        let content = ListWorkflowsHandler::render(&response);
        output.push_str(
            &BoxComponent::new(
                ContentComponent::new("Workflows".to_string(), content),
                terminal,
            )
            .render(),
        );
        Ok(())
    }

    fn execute_list_actions(
        &self,
        args: super::list_actions_args::ListActionsArgs,
        terminal: &dyn Terminal,
        output: &mut String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response =
            ListActionsHandler::handle(&*self.list_actions_port, args.path().to_path_buf())?;
        let content = ListActionsHandler::render(&response);
        output.push_str(
            &BoxComponent::new(
                ContentComponent::new("Actions".to_string(), content),
                terminal,
            )
            .render(),
        );
        Ok(())
    }
}
