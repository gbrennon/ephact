/// Failure while reading or writing persisted settings.
///
#[derive(Debug)]
pub enum SettingsStoreError {
    /// A settings file read failed.
    Read(String),
    /// Settings could not be serialized or written.
    Write(String),
    /// Persisted settings could not be parsed or validated.
    Parse(String),
    /// The configured settings path could not be resolved.
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
