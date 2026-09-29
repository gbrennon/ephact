use std::{path::PathBuf, sync::Mutex};

use ephact::{
    application::{errors::SettingsStoreError, ports::outbound::SettingsStorePort},
    domain::Settings,
};

pub struct FakeSettingsStore {
    settings: Mutex<Settings>,
    writes: Mutex<Vec<Settings>>,
    failure: Mutex<Option<String>>,
    path: PathBuf,
}

impl FakeSettingsStore {
    pub fn new(settings: Settings) -> Self {
        Self {
            settings: Mutex::new(settings),
            writes: Mutex::new(Vec::new()),
            failure: Mutex::new(None),
            path: PathBuf::from("/tmp/ephact-test-config.toml"),
        }
    }

    pub fn failing(settings: Settings, message: &str) -> Self {
        let store = Self::new(settings);
        *store.failure.lock().expect("failure lock") = Some(message.to_string());
        store
    }

    pub fn writes(&self) -> Vec<Settings> {
        self.writes.lock().expect("writes lock").clone()
    }
}

impl SettingsStorePort for FakeSettingsStore {
    fn read_settings(&self) -> Result<Settings, SettingsStoreError> {
        Ok(self.settings.lock().expect("settings lock").clone())
    }

    fn write_settings(&self, settings: &Settings) -> Result<(), SettingsStoreError> {
        if let Some(message) = self.failure.lock().expect("failure lock").clone() {
            return Err(SettingsStoreError::Write(message));
        }
        *self.settings.lock().expect("settings lock") = settings.clone();
        self.writes
            .lock()
            .expect("writes lock")
            .push(settings.clone());
        Ok(())
    }

    fn config_path(&self) -> PathBuf {
        self.path.clone()
    }
}
