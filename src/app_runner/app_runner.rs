use std::{error::Error, ffi::OsString};

use ephact::{
    application::ports::outbound::SettingsStorePort,
    infrastructure::{
        CargoProjectBrandingStore, Container, HostSshForwardingSettingsPort,
        containers::HostSshForwardingConfig, logging::stderr_filter::StderrFilter,
    },
    presentation::{
        cli::{CliParser, run_progress_handler::RunProgressHandler},
        composition_root::CompositionRoot,
    },
};

use super::ConfigFactory;

pub struct AppRunner {
    config_factory: ConfigFactory,
}

impl AppRunner {
    pub fn new() -> Self {
        Self {
            config_factory: ConfigFactory::from_environment(),
        }
    }

    pub fn run_application(&self) {
        let stderr_filter = StderrFilter::install();
        let result = self.run_application_impl();
        stderr_filter.restore();
        Self::finish(result);
    }

    fn run_application_impl(&self) -> Result<(), Box<dyn Error>> {
        let args: Vec<OsString> = std::env::args_os().collect();
        let invocation = CliParser::try_parse_invocation_from(&args).ok();
        let settings_store = self.config_factory.create_settings_store()?;
        let settings = settings_store.read_settings()?;
        let persisted_forward_ssh = settings_store.read_forward_ssh()?;
        let ssh_forwarding = Self::resolve_ssh_forwarding_for_invocation(
            invocation.as_ref(),
            args.len(),
            &settings,
            persisted_forward_ssh,
        );
        let verbose = invocation
            .as_ref()
            .is_some_and(CliParser::is_verbose_requested);
        let (progress_reporter, progress_stream) = RunProgressHandler::with_tui_stream(verbose);
        let branding_store = CargoProjectBrandingStore::from_metadata(
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_DESCRIPTION"),
            env!("CARGO_PKG_VERSION"),
            ephact::PROJECT_EMBLEM,
        );
        let container = if invocation
            .as_ref()
            .is_some_and(CliParser::is_discovery_command)
        {
            Container::build_for_discovery(
                Some(Box::new(progress_reporter)),
                Box::new(branding_store),
            )
        } else {
            Container::build_with_branding_and_ssh_forwarding(
                Some(Box::new(progress_reporter)),
                Box::new(branding_store),
                ssh_forwarding,
            )
        };
        let app = CompositionRoot::compose_with_tui_progress_and_settings(
            container,
            progress_stream,
            settings,
            settings_store.clone(),
            settings_store,
        );
        app.run(args)
    }

    fn resolve_ssh_forwarding_for_invocation(
        invocation: Option<&CliParser>,
        argument_count: usize,
        settings: &ephact::domain::Settings,
        persisted_forward_ssh: bool,
    ) -> HostSshForwardingConfig {
        let allow_network =
            settings.allow_network() || invocation.is_some_and(CliParser::is_network_allowed);
        let execution_requested = invocation
            .is_some_and(|parser| parser.is_run_command() || parser.is_tui_command())
            || (argument_count == 1
                && settings.default_interface() == ephact::domain::InterfaceMode::Tui);
        Self::resolve_ssh_forwarding(
            persisted_forward_ssh,
            allow_network,
            execution_requested,
            invocation.is_some_and(CliParser::is_ssh_forwarding_requested),
        )
    }

    fn resolve_ssh_forwarding(
        persisted_forward_ssh: bool,
        allow_network: bool,
        execution_requested: bool,
        explicit_forward_ssh: bool,
    ) -> HostSshForwardingConfig {
        let forwarding_enabled =
            execution_requested && allow_network && (persisted_forward_ssh || explicit_forward_ssh);
        if forwarding_enabled {
            HostSshForwardingConfig::enabled()
        } else {
            HostSshForwardingConfig::disabled()
        }
    }

    fn finish(result: Result<(), Box<dyn Error>>) {
        match result {
            Ok(()) => std::process::exit(0),
            Err(error) => {
                eprintln!("Error: {error}");
                std::process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parser(values: &[&str]) -> CliParser {
        let arguments = values.iter().map(OsString::from).collect::<Vec<_>>();
        CliParser::try_parse_invocation_from(&arguments).expect("arguments should parse")
    }

    #[test]
    fn persisted_forwarding_is_disabled_without_network_access() {
        let invocation = parser(&["ephact", "run"]);
        let config = AppRunner::resolve_ssh_forwarding(
            true,
            invocation.is_network_allowed(),
            invocation.is_run_command(),
            invocation.is_ssh_forwarding_requested(),
        );

        assert!(!config.is_enabled());
    }

    #[test]
    fn explicit_run_forwarding_is_enabled_with_network_access() {
        let invocation = parser(&["ephact", "run", "--allow-network", "--forward-ssh"]);
        let config = AppRunner::resolve_ssh_forwarding(
            false,
            invocation.is_network_allowed(),
            invocation.is_run_command(),
            invocation.is_ssh_forwarding_requested(),
        );

        assert!(config.is_enabled());
    }

    #[test]
    fn persisted_forwarding_is_enabled_for_tui_commands() {
        let invocation = parser(&["ephact", "tui"]);
        let config = AppRunner::resolve_ssh_forwarding(
            true,
            true,
            invocation.is_tui_command(),
            invocation.is_ssh_forwarding_requested(),
        );

        assert!(config.is_enabled());
    }

    #[test]
    fn persisted_forwarding_is_enabled_for_cli_tui_commands() {
        let invocation = parser(&["ephact", "cli", "tui"]);
        let config = AppRunner::resolve_ssh_forwarding(
            true,
            true,
            invocation.is_tui_command(),
            invocation.is_ssh_forwarding_requested(),
        );

        assert!(config.is_enabled());
    }

    #[test]
    fn persisted_forwarding_is_enabled_for_default_tui() {
        let config = AppRunner::resolve_ssh_forwarding(true, true, true, false);

        assert!(config.is_enabled());
    }

    #[test]
    fn persisted_forwarding_is_disabled_for_unrelated_commands() {
        let invocation = parser(&["ephact", "settings", "show"]);
        let config = AppRunner::resolve_ssh_forwarding(
            true,
            true,
            invocation.is_run_command() || invocation.is_tui_command(),
            invocation.is_ssh_forwarding_requested(),
        );

        assert!(!config.is_enabled());
    }
}
