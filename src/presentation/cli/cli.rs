use std::ffi::OsString;

use super::{
    cli_parser::CliParser, command::Command, list_actions_handler::ListActionsHandler,
    list_workflows_handler::ListWorkflowsHandler, run_handler::RunHandler,
};
use crate::application::ports::inbound::{
    list_actions_port::ListActionsPort, list_workflows_port::ListWorkflowsPort,
    run_all_workflows_port::RunAllWorkflowsPort, run_workflow_port::RunWorkflowPort,
    show_project_branding_info_port::ShowProjectBrandingInfoPort,
};
use crate::presentation::components::{
    banner::Banner, box_component::BoxComponent, content::ContentComponent,
    terminal::SystemTerminal,
};

pub struct Cli {
    run_workflow_port: Box<dyn RunWorkflowPort>,
    run_all_workflows_port: Box<dyn RunAllWorkflowsPort>,
    list_workflows_port: Box<dyn ListWorkflowsPort>,
    list_actions_port: Box<dyn ListActionsPort>,
    show_project_branding_info_port: Box<dyn ShowProjectBrandingInfoPort>,
}

impl Cli {
    pub fn new(
        run_workflow_port: Box<dyn RunWorkflowPort>,
        run_all_workflows_port: Box<dyn RunAllWorkflowsPort>,
        list_workflows_port: Box<dyn ListWorkflowsPort>,
        list_actions_port: Box<dyn ListActionsPort>,
        show_project_branding_info_port: Box<dyn ShowProjectBrandingInfoPort>,
    ) -> Self {
        Self {
            run_workflow_port,
            run_all_workflows_port,
            list_workflows_port,
            list_actions_port,
            show_project_branding_info_port,
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
        terminal: &dyn crate::presentation::components::terminal::Terminal,
    ) -> Result<String, Box<dyn std::error::Error>>
    where
        I: IntoIterator<Item = T>,
        T: Into<OsString> + Clone,
    {
        let branding = self.show_project_branding_info_port.execute()?;
        let mut output = BoxComponent::new(Banner::new(&branding), terminal).render();

        let parsed = CliParser::try_parse_from(args);
        let cli = match parsed {
            Ok(cli) => cli,
            Err(e) => return render_parse_error(e),
        };
        self.execute_command(cli.command, terminal, &mut output)?;
        Ok(output)
    }

    fn execute_command(
        &self,
        command: Command,
        terminal: &dyn crate::presentation::components::terminal::Terminal,
        output: &mut String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        match command {
            Command::Run(args) => self.execute_run(*args, terminal, output),
            Command::ListWorkflows(args) => self.execute_list_workflows(*args, terminal, output),
            Command::ListActions(args) => self.execute_list_actions(*args, terminal, output),
        }
    }

    fn execute_run(
        &self,
        args: super::run_args::RunArgs,
        terminal: &dyn crate::presentation::components::terminal::Terminal,
        output: &mut String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (summary, success) = RunHandler::handle_with_output(
            args,
            &*self.run_workflow_port,
            &*self.run_all_workflows_port,
            terminal,
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
        terminal: &dyn crate::presentation::components::terminal::Terminal,
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
        terminal: &dyn crate::presentation::components::terminal::Terminal,
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

fn render_parse_error(e: clap::error::Error) -> Result<String, Box<dyn std::error::Error>> {
    if e.kind() == clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        || !e.use_stderr()
    {
        Ok(e.to_string())
    } else {
        Err(e.to_string().into())
    }
}
