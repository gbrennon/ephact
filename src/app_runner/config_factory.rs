use std::{env, ffi::OsString, path::PathBuf, sync::Arc};

use crate::{application::errors::SettingsStoreError, infrastructure::TomlSettingsStore};

pub(super) struct ConfigFactory {
    home: Option<OsString>,
}

impl ConfigFactory {
    pub(super) fn from_environment() -> Self {
        Self::from_home(env::var_os("HOME"))
    }

    pub(super) fn from_home(home: Option<OsString>) -> Self {
        Self { home }
    }

    pub(super) fn create_settings_store(
        &self,
    ) -> Result<Arc<TomlSettingsStore>, SettingsStoreError> {
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
