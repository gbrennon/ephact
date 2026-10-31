use std::{error::Error, ffi::OsString};

use super::config_factory::ConfigFactory;
use crate::{
    application::ports::outbound::SettingsStorePort,
    domain::{InterfaceMode, Settings},
    infrastructure::{
        CargoProjectBrandingStore, Container, HostSshForwardingSettingsPort,
        containers::HostSshForwardingConfig,
    },
    presentation::{
        cli::{CliParser, run_progress_handler::RunProgressHandler},
        composition_root::CompositionRoot,
    },
};

pub enum ApplicationRunner {}

impl ApplicationRunner {
    /// Runs the composed application through the binary's process boundary.
    pub fn run_application() -> Result<(), Box<dyn Error>> {
        let args: Vec<OsString> = std::env::args_os().collect();
        let invocation = CliParser::try_parse_invocation_from(&args).ok();
        let settings_store = ConfigFactory::from_environment().create_settings_store()?;
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
            crate::PROJECT_EMBLEM,
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

    pub fn resolve_ssh_forwarding(
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

    fn resolve_ssh_forwarding_for_invocation(
        invocation: Option<&CliParser>,
        argument_count: usize,
        settings: &Settings,
        persisted_forward_ssh: bool,
    ) -> HostSshForwardingConfig {
        let allow_network =
            settings.allow_network() || invocation.is_some_and(CliParser::is_network_allowed);
        let execution_requested = invocation
            .is_some_and(|parser| parser.is_run_command() || parser.is_tui_command())
            || (argument_count == 1 && settings.default_interface() == InterfaceMode::Tui);
        Self::resolve_ssh_forwarding(
            persisted_forward_ssh,
            allow_network,
            execution_requested,
            invocation.is_some_and(CliParser::is_ssh_forwarding_requested),
        )
    }
}
