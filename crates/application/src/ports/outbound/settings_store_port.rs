use crate::{domain::Settings, errors::SettingsStoreError};

pub trait SettingsStorePort: Send + Sync {
    fn read_settings(&self) -> Result<Settings, SettingsStoreError>;
    fn write_settings(&self, settings: &Settings) -> Result<(), SettingsStoreError>;
    fn config_path(&self) -> std::path::PathBuf;
}
