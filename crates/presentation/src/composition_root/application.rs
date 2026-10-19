use super::super::cli::Cli;
use crate::{components::terminal::Terminal, infrastructure::HostSshForwardingSettingsPort};

/// Fully-wired presentation layer, returned by [`super::CompositionRoot::compose`].
pub struct Application {
    cli: Cli,
}

impl Application {
    pub fn new(cli: Cli) -> Self {
        Self { cli }
    }

    pub fn with_settings(
        self,
        settings: crate::domain::Settings,
        store: std::sync::Arc<dyn crate::application::ports::outbound::SettingsStorePort>,
    ) -> Self {
        Self {
            cli: self.cli.with_settings(settings, store),
        }
    }

    pub fn with_host_ssh_settings(
        self,
        settings: std::sync::Arc<dyn HostSshForwardingSettingsPort>,
    ) -> Self {
        Self {
            cli: self.cli.with_host_ssh_settings(settings),
        }
    }

    pub fn cli(&self) -> &Cli {
        &self.cli
    }

    pub fn run<I, T>(self, args: I) -> Result<(), Box<dyn std::error::Error>>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        self.cli.run(args)
    }
    pub fn run_with_terminal<I, T>(
        self,
        args: I,
        terminal: &dyn Terminal,
    ) -> Result<String, Box<dyn std::error::Error>>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        self.cli.run_with_terminal(args, terminal)
    }
}

impl AsRef<Cli> for Application {
    fn as_ref(&self) -> &Cli {
        &self.cli
    }
}

impl std::ops::Deref for Application {
    type Target = Cli;

    fn deref(&self) -> &Self::Target {
        &self.cli
    }
}
