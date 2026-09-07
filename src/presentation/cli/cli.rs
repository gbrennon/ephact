use std::{ffi::OsString, io::Write};

use clap::Parser;

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
        let branding = self.show_project_branding_info_port.execute()?;
        print!(
            "{}",
            BoxComponent::new(Banner::new(&branding), &terminal).render()
        );

        let parsed = CliParser::try_parse_from(args);
        let cli = match parsed {
            Ok(cli) => cli,
            Err(e) => return render_parse_error(e),
        };
        match cli.command {
            Command::Run(args) => RunHandler::handle(
                *args,
                &*self.run_workflow_port,
                &*self.run_all_workflows_port,
                &terminal,
            ),
            Command::ListWorkflows(args) => {
                let content = ListWorkflowsHandler::handle(*args, &*self.list_workflows_port)?;
                print!(
                    "{}",
                    BoxComponent::new(
                        ContentComponent::new("Workflows".to_string(), content),
                        &terminal,
                    )
                    .render()
                );
                Ok(())
            }
            Command::ListActions(args) => {
                let content = ListActionsHandler::handle(*args, &*self.list_actions_port)?;
                print!(
                    "{}",
                    BoxComponent::new(
                        ContentComponent::new("Actions".to_string(), content),
                        &terminal,
                    )
                    .render()
                );
                Ok(())
            }
        }
    }
}

fn render_parse_error(e: clap::error::Error) -> Result<(), Box<dyn std::error::Error>> {
    if e.kind() == clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
        || !e.use_stderr()
    {
        let mut stdout = std::io::stdout();
        let _ = write!(stdout, "{e}");
        let _ = stdout.flush();
        return Ok(());
    }
    let _ = write!(std::io::stderr(), "{e}");
    Err(e.to_string().into())
}
