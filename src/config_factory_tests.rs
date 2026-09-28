use std::ffi::OsString;

use super::create_settings_store_from_home;

#[test]
fn creates_store_from_home_directory() {
    let store = create_settings_store_from_home(Some(OsString::from("/tmp/ephact-home")))
        .expect("home directory should produce a settings store");

    assert_eq!(
        store.path().to_string_lossy(),
        "/tmp/ephact-home/.config/ephact/config.toml"
    );
}

#[test]
fn rejects_missing_home_directory() {
    let error = match create_settings_store_from_home(None) {
        Ok(_) => panic!("missing HOME should fail"),
        Err(error) => error,
    };

    assert_eq!(
        error.to_string(),
        "settings path could not be resolved: HOME is not set"
    );
}
