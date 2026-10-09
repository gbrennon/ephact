use std::{
    env, fs,
    path::{Path, PathBuf},
};

use super::{
    super::atomic_write, host_ssh_forwarding_settings_port::HostSshForwardingSettingsPort,
    toml_settings::TomlSettings,
};
use crate::{
    application::{errors::SettingsStoreError, ports::outbound::SettingsStorePort},
    domain::Settings,
};

/// Persists application settings to a TOML file on disk.
pub struct TomlSettingsStore {
    path: PathBuf,
}

impl TomlSettingsStore {
    /// Resolves the configuration path from the user's HOME environment variable.
    pub fn from_environment() -> Result<Self, SettingsStoreError> {
        let home = env::var_os("HOME")
            .ok_or_else(|| SettingsStoreError::Path("HOME is not set".to_string()))?;
        Ok(Self::new(
            PathBuf::from(home).join(".config/ephact/config.toml"),
        ))
    }

    /// Creates a store targeting the specified path.
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    /// Returns the target configuration file path.
    pub fn path(&self) -> &Path {
        &self.path
    }

    fn parent_path(&self) -> Result<&Path, SettingsStoreError> {
        self.path
            .parent()
            .ok_or_else(|| SettingsStoreError::Path("settings path has no parent".to_string()))
    }

    fn read_toml_settings(&self) -> Result<TomlSettings, SettingsStoreError> {
        let contents = match fs::read_to_string(&self.path) {
            Ok(contents) => contents,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(TomlSettings::from(&Settings::default()));
            }
            Err(error) => return Err(SettingsStoreError::Read(error.to_string())),
        };
        toml::from_str::<TomlSettings>(&contents)
            .map_err(|error| SettingsStoreError::Parse(error.to_string()))
    }

    fn write_toml_settings(&self, settings: &TomlSettings) -> Result<(), SettingsStoreError> {
        let parent = self.parent_path()?;
        fs::create_dir_all(parent).map_err(|error| SettingsStoreError::Write(error.to_string()))?;
        let contents = toml::to_string_pretty(settings)
            .map_err(|error| SettingsStoreError::Write(error.to_string()))?;
        atomic_write::write(&self.path, contents)
            .map_err(|error| SettingsStoreError::Write(error.to_string()))
    }
}

impl SettingsStorePort for TomlSettingsStore {
    fn read_settings(&self) -> Result<Settings, SettingsStoreError> {
        let toml_settings = self.read_toml_settings()?;
        Settings::try_from(toml_settings).map_err(SettingsStoreError::Parse)
    }

    fn write_settings(&self, settings: &Settings) -> Result<(), SettingsStoreError> {
        let forward_ssh = match fs::metadata(&self.path) {
            Ok(_) => self.read_toml_settings()?.forward_ssh(),
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::NotFound | std::io::ErrorKind::NotADirectory
                ) =>
            {
                false
            }
            Err(error) => return Err(SettingsStoreError::Read(error.to_string())),
        };
        let toml_settings = TomlSettings::from(settings).with_forward_ssh(forward_ssh);
        self.write_toml_settings(&toml_settings)
    }

    fn config_path(&self) -> PathBuf {
        self.path.clone()
    }
}

impl HostSshForwardingSettingsPort for TomlSettingsStore {
    fn read_forward_ssh(&self) -> Result<bool, SettingsStoreError> {
        Ok(self.read_toml_settings()?.forward_ssh())
    }

    fn write_forward_ssh(&self, enabled: bool) -> Result<(), SettingsStoreError> {
        let toml_settings = match fs::metadata(&self.path) {
            Ok(_) => self.read_toml_settings()?.with_forward_ssh(enabled),
            Err(error)
                if matches!(
                    error.kind(),
                    std::io::ErrorKind::NotFound | std::io::ErrorKind::NotADirectory
                ) =>
            {
                TomlSettings::from(&Settings::default()).with_forward_ssh(enabled)
            }
            Err(error) => return Err(SettingsStoreError::Read(error.to_string())),
        };
        self.write_toml_settings(&toml_settings)
    }
    fn write_settings_with_forward_ssh(
        &self,
        settings: &Settings,
        forward_ssh: bool,
    ) -> Result<(), SettingsStoreError> {
        let toml_settings = TomlSettings::from(settings).with_forward_ssh(forward_ssh);
        self.write_toml_settings(&toml_settings)
    }

    fn reset_settings(&self) -> Result<(), SettingsStoreError> {
        let toml_settings = TomlSettings::from(&Settings::default()).with_forward_ssh(false);
        self.write_toml_settings(&toml_settings)
    }
}
