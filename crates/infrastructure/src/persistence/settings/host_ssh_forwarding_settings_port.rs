use crate::{application::errors::SettingsStoreError, domain::Settings};

pub trait HostSshForwardingSettingsPort: Send + Sync {
    fn read_forward_ssh(&self) -> Result<bool, SettingsStoreError>;

    fn write_forward_ssh(&self, enabled: bool) -> Result<(), SettingsStoreError>;

    fn write_settings_with_forward_ssh(
        &self,
        settings: &Settings,
        forward_ssh: bool,
    ) -> Result<(), SettingsStoreError>;

    fn reset_settings(&self) -> Result<(), SettingsStoreError>;
}
