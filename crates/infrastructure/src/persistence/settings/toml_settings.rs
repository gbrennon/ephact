use serde::{Deserialize, Serialize};

use super::marker_serializer;
use crate::domain::{InterfaceMode, Settings};

#[derive(Debug, Clone, Copy, Default, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum TomlInterfaceMode {
    #[default]
    Tui,
    Cli,
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub(super) struct TomlSettings {
    #[serde(skip_serializing_if = "Self::is_default_interface")]
    default_interface: TomlInterfaceMode,
    #[serde(skip_serializing_if = "Self::is_default_bool")]
    allow_repo_writes: bool,
    #[serde(skip_serializing_if = "Self::is_default_bool")]
    allow_real_container: bool,
    #[serde(skip_serializing_if = "Self::is_default_bool")]
    allow_real_fetcher: bool,
    #[serde(skip_serializing_if = "Self::is_default_bool")]
    allow_network: bool,
    #[serde(skip_serializing_if = "Self::is_default_bool")]
    preserve: bool,
    #[serde(skip_serializing_if = "Self::is_default_bool")]
    verbose: bool,
    #[serde(skip_serializing_if = "Self::is_default_bool")]
    interactive: bool,
    #[serde(skip_serializing_if = "Self::is_default_bool")]
    all_workflows: bool,
    #[serde(default = "default_failure_log_retention_hours")]
    #[serde(skip_serializing_if = "Self::is_default_failure_log_retention_hours")]
    failure_log_retention_hours: u64,
    #[serde(default = "marker_serializer::default_marker")]
    marker: String,
}

fn default_failure_log_retention_hours() -> u64 {
    Settings::DEFAULT_FAILURE_LOG_RETENTION_HOURS
}

impl TomlSettings {
    fn is_default_interface(value: &TomlInterfaceMode) -> bool {
        matches!(value, TomlInterfaceMode::Tui)
    }

    fn is_default_bool(value: &bool) -> bool {
        !value
    }

    fn is_default_failure_log_retention_hours(value: &u64) -> bool {
        *value == Settings::DEFAULT_FAILURE_LOG_RETENTION_HOURS
    }
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
            failure_log_retention_hours: settings.failure_log_retention_hours(),
            marker: marker_serializer::serialize(settings.marker()),
        }
    }
}

impl TryFrom<TomlSettings> for Settings {
    type Error = String;

    fn try_from(settings: TomlSettings) -> Result<Self, Self::Error> {
        let default_interface = match settings.default_interface {
            TomlInterfaceMode::Tui => InterfaceMode::Tui,
            TomlInterfaceMode::Cli => InterfaceMode::Cli,
        };
        let marker = marker_serializer::deserialize(&settings.marker);
        let settings = Settings::default()
            .with_default_interface(default_interface)
            .with_allow_repo_writes(settings.allow_repo_writes)
            .with_allow_real_container(settings.allow_real_container)
            .with_allow_real_fetcher(settings.allow_real_fetcher)
            .with_allow_network(settings.allow_network)
            .with_preserve(settings.preserve)
            .with_verbose(settings.verbose)
            .with_interactive(settings.interactive)
            .with_all_workflows(settings.all_workflows)
            .with_failure_log_retention_hours(settings.failure_log_retention_hours)?;
        Ok(settings.with_marker(marker))
    }
}
