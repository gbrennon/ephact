use std::fs;

use super::write;

#[test]
fn writes_content_to_target_path() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let target = directory.path().join("settings.toml");

    write(&target, "value = 1").expect("atomic write");

    assert_eq!(
        fs::read_to_string(target).expect("written file"),
        "value = 1"
    );
}

#[test]
fn reports_missing_parent_without_leaving_temporary_file() {
    let directory = tempfile::tempdir().expect("temporary directory");
    let target = directory.path().join("missing").join("settings.toml");

    assert!(write(&target, "value = 1").is_err());
    assert!(!directory.path().join("missing").exists());
}
