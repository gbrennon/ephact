#[derive(Debug)]
pub enum SettingsStoreError {
    Read(String),
    Write(String),
    Parse(String),
    Path(String),
}

impl_application_error!(
    SettingsStoreError,
    |error: &SettingsStoreError| match error {
        SettingsStoreError::Read(message) => format!("settings file could not be read: {message}"),
        SettingsStoreError::Write(message) =>
            format!("settings file could not be written: {message}"),
        SettingsStoreError::Parse(message) =>
            format!("settings file contains invalid TOML: {message}"),
        SettingsStoreError::Path(message) =>
            format!("settings path could not be resolved: {message}"),
    },
);

impl std::error::Error for SettingsStoreError {}
