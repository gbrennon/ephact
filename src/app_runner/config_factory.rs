use std::{env, ffi::OsString, path::PathBuf, sync::Arc};

use ephact::{application::errors::SettingsStoreError, infrastructure::TomlSettingsStore};

pub struct ConfigFactory {
    home: Option<OsString>,
}

impl ConfigFactory {
    pub fn from_environment() -> Self {
        Self::from_home(env::var_os("HOME"))
    }

    pub fn from_home(home: Option<OsString>) -> Self {
        Self { home }
    }

    pub fn create_settings_store(&self) -> Result<Arc<TomlSettingsStore>, SettingsStoreError> {
        let home = self
            .home
            .as_ref()
            .map(PathBuf::from)
            .ok_or_else(|| SettingsStoreError::Path("HOME is not set".to_string()))?;
        Ok(Arc::new(TomlSettingsStore::new(
            home.join(".config/ephact/config.toml"),
        )))
    }
}
