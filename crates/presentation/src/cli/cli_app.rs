use std::{ffi::OsString, sync::Arc};

use super::{
    super::components::{
        banner::Banner,
        box_component::BoxComponent,
        content::ContentComponent,
        terminal::{SystemTerminal, Terminal},
    },
    cli_parser::CliParser,
    command::Command,
    run_progress_handler::TuiProgressStream,
    settings_command::{SettingName, SettingsCommand},
};
use crate::{
    application::ports::{
        inbound::{
            list_actions_port::ListActionsPort, list_workflows_port::ListWorkflowsPort,
            run_all_workflows_port::RunAllWorkflowsPort, run_workflow_port::RunWorkflowPort,
            show_project_branding_info_port::ShowProjectBrandingInfoPort,
        },
        outbound::{RunInputsDiscovererPort, SettingsStorePort},
    },
    domain::{InterfaceMode, Settings},
    handlers::{
        DiagnosticStores, ListActionsHandler, ListWorkflowsHandler, PreflightPorts, RunHandler,
    },
    tui::TuiRunner,
};

pub struct Cli {
    run_workflow_port: Arc<dyn RunWorkflowPort>,
    run_all_workflows_port: Box<dyn RunAllWorkflowsPort>,
    discover_run_inputs_port: Arc<dyn RunInputsDiscovererPort>,
    list_workflows_port: Arc<dyn ListWorkflowsPort>,
    list_actions_port: Arc<dyn ListActionsPort>,
    show_project_branding_info_port: Box<dyn ShowProjectBrandingInfoPort>,
    failure_log_error_store: crate::infrastructure::logging::FailureLogErrorStore,
    failure_log_path_store: crate::infrastructure::logging::FailureLogPathStore,
    tui_runner: TuiRunner,
    settings: Settings,
    settings_store: Option<Arc<dyn SettingsStorePort>>,
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
        let discover_run_inputs_port: Arc<dyn RunInputsDiscovererPort> =
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
            settings: Settings::default(),
            settings_store: None,
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

    pub fn with_settings(
        mut self,
        settings: Settings,
        settings_store: Arc<dyn SettingsStorePort>,
    ) -> Self {
        self.settings = settings.clone();
        self.settings_store = Some(settings_store.clone());
        self.tui_runner = self
            .tui_runner
            .with_settings(settings, Some(settings_store));
        self
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

    fn is_tui_command(args: &[OsString]) -> bool {
        args.get(1).is_some_and(|arg| arg == "tui")
    }
    async fn execute_cli(
        &self,
        args: Vec<OsString>,
        terminal: &dyn Terminal,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let force_cli = Self::is_cli_command(&args);
        let parse_args = Self::remove_cli_command(args);
        let parsed = CliParser::try_parse_from(parse_args);
        let cli = match parsed {
            Ok(cli) => cli,
            Err(error) => {
                self.show_project_branding_info_port.execute()?;
                return Self::render_parse_error(error);
            }
        };
        self.dispatch_parsed_cli(cli, terminal, force_cli).await
    }

    fn is_cli_command(args: &[OsString]) -> bool {
        args.get(1).is_some_and(|arg| arg == "cli")
    }

    fn remove_cli_command(mut args: Vec<OsString>) -> Vec<OsString> {
        if Self::is_cli_command(&args) {
            args.remove(1);
        }
        args
    }

    async fn dispatch_parsed_cli(
        &self,
        cli: CliParser,
        terminal: &dyn Terminal,
        force_cli: bool,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if !cli.has_explicit_command() {
            return if force_cli {
                Ok(CliParser::build_command().render_long_help().to_string())
            } else {
                self.execute_default_interface().await
            };
        }
        self.execute_explicit_command(cli.command(), terminal).await
    }

    async fn execute_default_interface(&self) -> Result<String, Box<dyn std::error::Error>> {
        if self.settings.default_interface() == InterfaceMode::Tui {
            self.tui_runner.run().await?;
            return Ok(String::new());
        }
        Ok(CliParser::build_command().render_long_help().to_string())
    }

    async fn execute_explicit_command(
        &self,
        command: Command,
        terminal: &dyn Terminal,
    ) -> Result<String, Box<dyn std::error::Error>> {
        match command {
            Command::Cli => Ok(CliParser::build_command().render_long_help().to_string()),
            Command::Settings(settings) => self.execute_settings(settings),
            command => {
                let branding = self.show_project_branding_info_port.execute()?;
                let mut output = BoxComponent::new(Banner::new(&branding), terminal).render();
                self.execute_command(command, terminal, &mut output).await?;
                Ok(output)
            }
        }
    }

    fn render_parse_error(e: clap::Error) -> Result<String, Box<dyn std::error::Error>> {
        if e.kind() == clap::error::ErrorKind::DisplayHelpOnMissingArgumentOrSubcommand
            || !e.use_stderr()
        {
            Ok(e.to_string())
        } else {
            Err(e.to_string().into())
        }
    }

    fn execute_settings(
        &self,
        command: SettingsCommand,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let store = self
            .settings_store
            .as_ref()
            .ok_or_else(|| std::io::Error::other("settings store is not configured"))?;
        match command {
            SettingsCommand::Show => self.show_settings(&**store),
            SettingsCommand::Reset => self.reset_settings(&**store),
            SettingsCommand::Set(arguments) => self.persist_setting(&**store, arguments),
        }
    }

    fn show_settings(
        &self,
        store: &dyn SettingsStorePort,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let settings = store.read_settings()?;
        Ok(Self::render_settings(&settings, &store.config_path()))
    }

    fn reset_settings(
        &self,
        store: &dyn SettingsStorePort,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let settings = Settings::default();
        store.write_settings(&settings)?;
        Ok(Self::render_settings(&settings, &store.config_path()))
    }

    fn persist_setting(
        &self,
        store: &dyn SettingsStorePort,
        arguments: super::settings_command::SettingsSetArgs,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let settings =
            Self::update_setting(store.read_settings()?, arguments.name(), arguments.value())?;
        store.write_settings(&settings)?;
        Ok(Self::render_settings(&settings, &store.config_path()))
    }

    fn update_setting(
        settings: Settings,
        name: SettingName,
        value: &str,
    ) -> Result<Settings, String> {
        match name {
            SettingName::DefaultInterface => match value {
                "tui" => Ok(settings.with_default_interface(InterfaceMode::Tui)),
                "cli" => Ok(settings.with_default_interface(InterfaceMode::Cli)),
                _ => Err("default-interface must be tui or cli".to_string()),
            },
            SettingName::AllowRepoWrites => {
                Self::update_bool(settings, value, Settings::with_allow_repo_writes)
            }
            SettingName::AllowRealContainer => {
                Self::update_bool(settings, value, Settings::with_allow_real_container)
            }
            SettingName::AllowRealFetcher => {
                Self::update_bool(settings, value, Settings::with_allow_real_fetcher)
            }
            SettingName::AllowNetwork => {
                Self::update_bool(settings, value, Settings::with_allow_network)
            }
            SettingName::Preserve => Self::update_bool(settings, value, Settings::with_preserve),
            SettingName::Verbose => Self::update_bool(settings, value, Settings::with_verbose),
            SettingName::Interactive => {
                Self::update_bool(settings, value, Settings::with_interactive)
            }
            SettingName::AllWorkflows => {
                Self::update_bool(settings, value, Settings::with_all_workflows)
            }
        }
    }

    fn update_bool(
        settings: Settings,
        value: &str,
        update: fn(Settings, bool) -> Settings,
    ) -> Result<Settings, String> {
        let parsed = match value {
            "true" => true,
            "false" => false,
            _ => return Err("setting value must be true or false".to_string()),
        };
        Ok(update(settings, parsed))
    }

    fn render_settings(settings: &Settings, path: &std::path::Path) -> String {
        format!(
            "Config: {}\n\
default-interface = {}\n\
allow-repo-writes = {}\n\
allow-real-container = {}\n\
allow-real-fetcher = {}\n\
allow-network = {}\n\
preserve = {}\n\
verbose = {}\n\
interactive = {}\n\
all-workflows = {}\n",
            path.display(),
            Self::interface_name(settings.default_interface()),
            settings.allow_repo_writes(),
            settings.allow_real_container(),
            settings.allow_real_fetcher(),
            settings.allow_network(),
            settings.preserve(),
            settings.verbose(),
            settings.interactive(),
            settings.all_workflows(),
        )
    }

    fn interface_name(mode: InterfaceMode) -> &'static str {
        match mode {
            InterfaceMode::Tui => "tui",
            InterfaceMode::Cli => "cli",
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
            Command::Settings(settings) => {
                output.push_str(&self.execute_settings(settings)?);
                Ok(())
            }
            Command::Tui => self.tui_runner.run().await,
            Command::Cli => {
                output.push_str(&CliParser::build_command().render_long_help().to_string());
                Ok(())
            }
        }
    }

    async fn execute_run(
        &self,
        args: super::run_args::RunArgs,
        terminal: &dyn Terminal,
        output: &mut String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut args = args;
        args.apply_settings(&self.settings);
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
