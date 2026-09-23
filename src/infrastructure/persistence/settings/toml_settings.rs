use serde::{Deserialize, Serialize};

use crate::domain::{InterfaceMode, Settings};

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum TomlInterfaceMode {
    #[default]
    Tui,
    Cli,
}

fn is_tui(value: &TomlInterfaceMode) -> bool {
    matches!(value, TomlInterfaceMode::Tui)
}

fn is_false(value: &bool) -> bool {
    !value
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub(super) struct TomlSettings {
    #[serde(skip_serializing_if = "is_tui")]
    default_interface: TomlInterfaceMode,
    #[serde(skip_serializing_if = "is_false")]
    allow_repo_writes: bool,
    #[serde(skip_serializing_if = "is_false")]
    allow_real_container: bool,
    #[serde(skip_serializing_if = "is_false")]
    allow_real_fetcher: bool,
    #[serde(skip_serializing_if = "is_false")]
    allow_network: bool,
    #[serde(skip_serializing_if = "is_false")]
    preserve: bool,
    #[serde(skip_serializing_if = "is_false")]
    verbose: bool,
    #[serde(skip_serializing_if = "is_false")]
    interactive: bool,
    #[serde(skip_serializing_if = "is_false")]
    all_workflows: bool,
}

impl From<&Settings> for TomlSettings {
    fn from(settings: &Settings) -> Self {
        Self {
            default_interface: match settings.default_interface() {
                InterfaceMode::Tui => TomlInterfaceMode::Tui,
                InterfaceMode::Cli => TomlInterfaceMode::Cli,
            },
            allow_repo_writes: settings.allow_repo_writes(),
            allow_real_container: settings.allow_real_container(),
            allow_real_fetcher: settings.allow_real_fetcher(),
            allow_network: settings.allow_network(),
            preserve: settings.preserve(),
            verbose: settings.verbose(),
            interactive: settings.interactive(),
            all_workflows: settings.all_workflows(),
        }
    }
}

impl From<TomlSettings> for Settings {
    fn from(settings: TomlSettings) -> Self {
        let default_interface = match settings.default_interface {
            TomlInterfaceMode::Tui => InterfaceMode::Tui,
            TomlInterfaceMode::Cli => InterfaceMode::Cli,
        };
        Settings::default()
            .with_default_interface(default_interface)
            .with_allow_repo_writes(settings.allow_repo_writes)
            .with_allow_real_container(settings.allow_real_container)
            .with_allow_real_fetcher(settings.allow_real_fetcher)
            .with_allow_network(settings.allow_network)
            .with_preserve(settings.preserve)
            .with_verbose(settings.verbose)
            .with_interactive(settings.interactive)
            .with_all_workflows(settings.all_workflows)
    }
}
