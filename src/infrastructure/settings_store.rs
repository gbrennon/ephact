use std::{
    env, fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{
    application::{errors::SettingsStoreError, ports::outbound::SettingsStorePort},
    domain::{InterfaceMode, Settings},
};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
struct TomlSettings {
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

pub struct TomlSettingsStore {
    path: PathBuf,
}

impl TomlSettingsStore {
    pub fn from_environment() -> Result<Self, SettingsStoreError> {
        let home = env::var_os("HOME")
            .ok_or_else(|| SettingsStoreError::Path("HOME is not set".to_string()))?;
        Ok(Self::new(
            PathBuf::from(home).join(".config/ephact/config.toml"),
        ))
    }

    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    fn parent_path(&self) -> Result<&Path, SettingsStoreError> {
        self.path
            .parent()
            .ok_or_else(|| SettingsStoreError::Path("settings path has no parent".to_string()))
    }

    fn serialize(&self, settings: &Settings) -> Result<String, SettingsStoreError> {
        toml::to_string_pretty(&TomlSettings::from(settings))
            .map_err(|error| SettingsStoreError::Write(error.to_string()))
    }

    fn temporary_path(&self) -> PathBuf {
        self.path
            .with_extension(format!("tmp-{}", uuid::Uuid::new_v4()))
    }

    fn replace_temporary_file(&self, temporary_path: &Path) -> Result<(), SettingsStoreError> {
        if let Err(error) = fs::rename(temporary_path, &self.path) {
            let _ = fs::remove_file(temporary_path);
            return Err(SettingsStoreError::Write(error.to_string()));
        }
        Ok(())
    }
}

impl SettingsStorePort for TomlSettingsStore {
    fn read_settings(&self) -> Result<Settings, SettingsStoreError> {
        let contents = match fs::read_to_string(&self.path) {
            Ok(contents) => contents,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Settings::default());
            }
            Err(error) => return Err(SettingsStoreError::Read(error.to_string())),
        };
        toml::from_str::<TomlSettings>(&contents)
            .map(Into::into)
            .map_err(|error| SettingsStoreError::Parse(error.to_string()))
    }

    fn write_settings(&self, settings: &Settings) -> Result<(), SettingsStoreError> {
        let parent = self.parent_path()?;
        fs::create_dir_all(parent).map_err(|error| SettingsStoreError::Write(error.to_string()))?;
        let contents = self.serialize(settings)?;
        let temporary_path = self.temporary_path();
        fs::write(&temporary_path, contents)
            .map_err(|error| SettingsStoreError::Write(error.to_string()))?;
        self.replace_temporary_file(&temporary_path)
    }

    fn config_path(&self) -> PathBuf {
        self.path.clone()
    }
}
