use serde::{Deserialize, Serialize};

use crate::domain::{
    InterfaceMode, Settings,
    value_objects::{Marker, MarkerPreset},
};

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
    #[serde(default = "default_marker")]
    marker: String,
}

impl TomlSettings {
    fn is_default_interface(value: &TomlInterfaceMode) -> bool {
        matches!(value, TomlInterfaceMode::Tui)
    }

    fn is_default_bool(value: &bool) -> bool {
        !value
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
            marker: marker_to_toml(settings.marker()),
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
            .with_marker(marker_from_toml(&settings.marker))
    }
}

fn default_marker() -> String {
    Marker::default().value().to_string()
}

fn marker_to_toml(marker: &Marker) -> String {
    match marker {
        Marker::Preset(preset) => preset.as_text().to_string(),
        Marker::CustomText(value) => format!("text:{value}"),
        Marker::ImagePath(value) => format!("image:{value}"),
        Marker::GifPath(value) => format!("gif:{value}"),
    }
}

fn marker_from_toml(value: &str) -> Marker {
    if let Some(value) = value.strip_prefix("text:") {
        return Marker::custom_text(value);
    }
    if let Some(value) = value.strip_prefix("image:") {
        return Marker::image_path(value);
    }
    if let Some(value) = value.strip_prefix("gif:") {
        return Marker::gif_path(value);
    }
    if let Some(preset) = MarkerPreset::ALL
        .into_iter()
        .find(|preset| preset.as_text() == value)
    {
        return Marker::preset(preset);
    }
    Marker::custom_text(value)
}
