#[path = "common/config_factory_process.rs"]
mod cli_process;

use cli_process::{run_with_home, run_without_home};
use tempfile::TempDir;

#[test]
fn settings_show_uses_config_path_under_home_directory() {
    let home = TempDir::new().expect("temporary home directory");
    let output = run_with_home(home.path(), &["settings", "show"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(
        output.status.success(),
        "status: {:?}\nstdout: {}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        stdout.contains(
            &home
                .path()
                .join(".config/ephact/config.toml")
                .display()
                .to_string()
        )
    );
}

#[test]
fn missing_home_directory_returns_an_error() {
    let output = run_without_home(&["settings", "show"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(
        output.status.code(),
        Some(1),
        "status: {:?}\nstdout: {}\nstderr: {}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(stderr.contains("settings path could not be resolved: HOME is not set"));
}
