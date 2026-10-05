use crate::{domain::Settings, errors::SettingsStoreError};

/// Persists and retrieves application [`Settings`].
pub trait SettingsStorePort: Send + Sync {
    /// Reads settings from the configured store.
    ///
    /// A missing settings file yields the default [`Settings`].
    ///
    /// # Errors
    ///
    /// Returns [`SettingsStoreError`] when settings cannot be read or parsed.
    fn read_settings(&self) -> Result<Settings, SettingsStoreError>;
    ///
    /// Serializes and writes `settings`, creating its parent directory as
    /// needed.
    ///
    /// # Errors
    ///
    /// Returns [`SettingsStoreError`] when the settings cannot be serialized
    /// or written.
    fn write_settings(&self, settings: &Settings) -> Result<(), SettingsStoreError>;
    ///
    /// Returns the path used by the store for its configuration.
    fn config_path(&self) -> std::path::PathBuf;
}
