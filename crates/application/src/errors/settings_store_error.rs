#[derive(Debug, thiserror::Error)]
pub enum SettingsStoreError {
    #[error("settings file could not be read: {0}")]
    Read(String),
    #[error("settings file could not be written: {0}")]
    Write(String),
    #[error("settings file contains invalid TOML: {0}")]
    Parse(String),
    #[error("settings path could not be resolved: {0}")]
    Path(String),
}
