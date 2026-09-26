#[cfg(test)]
mod tests {
    use ephact::{
        application::ports::outbound::SettingsStorePort,
        domain::{
            InterfaceMode, Settings,
            value_objects::{Marker, MarkerPreset},
        },
        infrastructure::TomlSettingsStore,
    };
    use tempfile::TempDir;

    #[test]
    fn missing_file_returns_default_settings() {
        let directory = TempDir::new().expect("temporary directory");
        let store = TomlSettingsStore::new(directory.path().join("config.toml"));

        let settings = store.read_settings().expect("missing settings are valid");

        assert_eq!(settings, Settings::default());
    }

    #[test]
    fn settings_round_trip_through_toml() {
        let directory = TempDir::new().expect("temporary directory");
        let store = TomlSettingsStore::new(directory.path().join("nested/config.toml"));
        let settings = Settings::default()
            .with_allow_network(true)
            .with_marker(Marker::preset(MarkerPreset::Chevron));

        store.write_settings(&settings).expect("settings write");
        let contents = std::fs::read_to_string(store.path()).expect("settings file");
        let reloaded = store.read_settings().expect("settings read");

        assert!(contents.contains("allow_network = true"));
        assert!(contents.contains("marker = \"❯\""));
        assert!(!contents.contains("allow_repo_writes"));
        assert_eq!(reloaded, settings);
    }

    #[test]
    fn directory_read_failure_is_reported() {
        let directory = TempDir::new().expect("temporary directory");
        let path = directory.path().join("config.toml");
        std::fs::create_dir(&path).expect("settings directory");
        let store = TomlSettingsStore::new(path);

        let error = store
            .read_settings()
            .expect_err("directory cannot be read as TOML");

        assert!(error.to_string().contains("could not be read"));
    }

    #[test]
    fn parent_write_failure_is_reported() {
        let directory = TempDir::new().expect("temporary directory");
        let parent = directory.path().join("parent");
        std::fs::write(&parent, "not a directory").expect("parent file");
        let store = TomlSettingsStore::new(parent.join("config.toml"));

        let error = store
            .write_settings(&Settings::default())
            .expect_err("file parent cannot be created");

        assert!(error.to_string().contains("could not be written"));
    }

    #[test]
    fn omitted_values_use_built_in_defaults() {
        let directory = TempDir::new().expect("temporary directory");
        let path = directory.path().join("config.toml");
        std::fs::write(&path, "allow_network = true\n").expect("settings file");
        let store = TomlSettingsStore::new(path);

        let settings = store.read_settings().expect("settings read");

        assert!(settings.allow_network());
        assert_eq!(settings.default_interface(), InterfaceMode::Tui);
        assert!(!settings.allow_repo_writes());
        assert_eq!(settings.marker(), &Marker::default());
    }

    #[test]
    fn custom_and_graphical_markers_round_trip_through_toml() {
        let directory = TempDir::new().expect("temporary directory");
        let store = TomlSettingsStore::new(directory.path().join("config.toml"));
        let settings = Settings::default().with_marker(Marker::custom_text("🚀"));

        store.write_settings(&settings).expect("settings write");
        let contents = std::fs::read_to_string(store.path()).expect("settings file");
        assert!(contents.contains("marker = \"text:🚀\""));
        assert_eq!(store.read_settings().expect("settings read"), settings);

        for (marker, encoded) in [
            (
                Marker::image_path("/tmp/marker.png"),
                "image:/tmp/marker.png",
            ),
            (Marker::gif_path("/tmp/marker.gif"), "gif:/tmp/marker.gif"),
        ] {
            let settings = Settings::default().with_marker(marker);
            store.write_settings(&settings).expect("settings write");
            let contents = std::fs::read_to_string(store.path()).expect("settings file");
            assert!(contents.contains(&format!("marker = \"{encoded}\"")));
            assert_eq!(store.read_settings().expect("settings read"), settings);
        }
    }

    #[test]
    fn malformed_toml_is_reported() {
        let directory = TempDir::new().expect("temporary directory");
        let path = directory.path().join("config.toml");
        std::fs::write(&path, "default_interface = [").expect("settings file");
        let store = TomlSettingsStore::new(path);

        let error = store
            .read_settings()
            .expect_err("malformed settings must fail");

        assert!(error.to_string().contains("invalid TOML"));
    }
}
