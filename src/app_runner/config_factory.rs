use std::{env, ffi::OsString, path::PathBuf, sync::Arc};

use ephact::{application::errors::SettingsStoreError, infrastructure::TomlSettingsStore};

pub struct ConfigFactory {
    _private: (),
}

impl ConfigFactory {
    pub fn create_settings_store() -> Result<Arc<TomlSettingsStore>, SettingsStoreError> {
        Self::create_settings_store_from_home(env::var_os("HOME"))
    }

    fn create_settings_store_from_home(
        home: Option<OsString>,
    ) -> Result<Arc<TomlSettingsStore>, SettingsStoreError> {
        let home = home
            .map(PathBuf::from)
            .ok_or_else(|| SettingsStoreError::Path("HOME is not set".to_string()))?;
        Ok(Arc::new(TomlSettingsStore::new(
            home.join(".config/ephact/config.toml"),
        )))
    }
}
