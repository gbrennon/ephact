use std::{
    env, fs,
    path::{Path, PathBuf},
};

use super::toml_settings::TomlSettings;
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
