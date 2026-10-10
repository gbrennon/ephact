#[cfg(test)]
mod tests {
    use std::fs;

    use ephact::{
        application::ports::outbound::SettingsStorePort,
        infrastructure::{HostSshForwardingSettingsPort, TomlSettingsStore},
    };
    use tempfile::tempdir;

    #[test]
    fn forward_ssh_defaults_to_false_and_round_trips_true() {
        let directory = tempdir().unwrap();
        let store = TomlSettingsStore::new(directory.path().join("config.toml"));

        assert!(!store.read_forward_ssh().unwrap());

        store.write_forward_ssh(true).unwrap();

        assert!(store.read_forward_ssh().unwrap());
        let contents = fs::read_to_string(store.path()).unwrap();
        assert!(contents.contains("forward_ssh = true"));
        assert!(!contents.contains("failure_log_retention_hours = 0"));
    }

    #[test]
    fn writing_domain_settings_preserves_forward_ssh() {
        let directory = tempdir().unwrap();
        let store = TomlSettingsStore::new(directory.path().join("config.toml"));
        store.write_forward_ssh(true).unwrap();

        SettingsStorePort::write_settings(&store, &ephact::domain::Settings::default()).unwrap();

        assert!(store.read_forward_ssh().unwrap());
    }

    #[test]
    fn resetting_forward_ssh_writes_the_safe_default() {
        let directory = tempdir().unwrap();
        let store = TomlSettingsStore::new(directory.path().join("config.toml"));
        store.write_forward_ssh(true).unwrap();

        store.write_forward_ssh(false).unwrap();

        assert!(!store.read_forward_ssh().unwrap());
        assert!(
            !fs::read_to_string(store.path())
                .unwrap()
                .contains("forward_ssh")
        );
    }
}
