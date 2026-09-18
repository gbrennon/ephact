use std::ffi::OsString;

use super::super::components::{
    banner::Banner,
    box_component::BoxComponent,
    content::ContentComponent,
    terminal::{SystemTerminal, Terminal},
};
use super::{
    cli_parser::CliParser,
    command::Command,
    list_actions_handler::ListActionsHandler,
    list_workflows_handler::ListWorkflowsHandler,
    run_handler::{DiagnosticStores, PreflightPorts, RunHandler},
};
use crate::application::ports::inbound::{
    list_actions_port::ListActionsPort, list_workflows_port::ListWorkflowsPort,
    run_all_workflows_port::RunAllWorkflowsPort, run_workflow_port::RunWorkflowPort,
    show_project_branding_info_port::ShowProjectBrandingInfoPort,
};
use crate::application::ports::outbound::DiscoverRunInputsPort;

pub struct Cli {
    run_workflow_port: Box<dyn RunWorkflowPort>,
    run_all_workflows_port: Box<dyn RunAllWorkflowsPort>,
    discover_run_inputs_port: Box<dyn DiscoverRunInputsPort>,
    list_workflows_port: Box<dyn ListWorkflowsPort>,
    list_actions_port: Box<dyn ListActionsPort>,
    show_project_branding_info_port: Box<dyn ShowProjectBrandingInfoPort>,
    failure_log_error_store: crate::infrastructure::logging::FailureLogErrorStore,
    failure_log_path_store: crate::infrastructure::logging::FailureLogPathStore,
}
pub type CliRunDependencies = (
    Box<dyn RunWorkflowPort>,
    Box<dyn RunAllWorkflowsPort>,
    Box<dyn DiscoverRunInputsPort>,
);
pub type CliListDependencies = (
    Box<dyn ListWorkflowsPort>,
    Box<dyn ListActionsPort>,
    Box<dyn ShowProjectBrandingInfoPort>,
);
pub type CliParts = (
    Box<dyn RunWorkflowPort>,
    Box<dyn RunAllWorkflowsPort>,
    Box<dyn DiscoverRunInputsPort>,
    Box<dyn ListWorkflowsPort>,
    Box<dyn ListActionsPort>,
    Box<dyn ShowProjectBrandingInfoPort>,
);

pub struct CliDependencies {
    run_dependencies: CliRunDependencies,
    list_dependencies: CliListDependencies,
}

impl CliDependencies {
    pub fn new(
        run_dependencies: CliRunDependencies,
        list_dependencies: CliListDependencies,
    ) -> Self {
        Self {
            run_dependencies,
            list_dependencies,
        }
    }

    fn into_parts(self) -> CliParts {
        let (run_workflow_port, run_all_workflows_port, discover_run_inputs_port) =
            self.run_dependencies;
        let (list_workflows_port, list_actions_port, show_project_branding_info_port) =
            self.list_dependencies;
        (
            run_workflow_port,
            run_all_workflows_port,
            discover_run_inputs_port,
            list_workflows_port,
            list_actions_port,
            show_project_branding_info_port,
        )
    }
}

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
        Self {
            run_workflow_port,
            run_all_workflows_port,
            discover_run_inputs_port,
            list_workflows_port,
            list_actions_port,
            show_project_branding_info_port,
            failure_log_error_store: failure_log_stores.error_store(),
            failure_log_path_store: failure_log_stores.path_store(),
        }
    }
}

impl Cli {
    pub fn run<I, T>(self, args: I) -> Result<(), Box<dyn std::error::Error>>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let terminal = SystemTerminal;
        let output = self.run_with_terminal(args, &terminal)?;
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
        let args: Vec<OsString> = args.into_iter().map(Into::into).collect();
        if Self::is_tui_command(&args) {
            crate::presentation::tui::Tui::run()?;
            return Ok(String::new());
        }
        self.execute_cli(args, terminal)
    }

    fn execute_cli(
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
        self.execute_command(command, terminal, &mut output)?;
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
    fn execute_command(
        &self,
        command: Command,
        terminal: &dyn Terminal,
        output: &mut String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match command {
            Command::Run(args) => self.execute_run(*args, terminal, output),
            Command::ListWorkflows(args) => self.execute_list_workflows(*args, terminal, output),
            Command::ListActions(args) => self.execute_list_actions(*args, terminal, output),
            Command::Tui => crate::presentation::tui::Tui::run(),
        }
    }

    fn execute_run(
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
        )?;
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
        let content = ListWorkflowsHandler::handle(args, &*self.list_workflows_port)?;
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
        let content = ListActionsHandler::handle(args, &*self.list_actions_port)?;
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
