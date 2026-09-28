use std::process::Command;

use tempfile::TempDir;

fn ephact_command(home: Option<&std::path::Path>, args: &[&str]) -> std::process::Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ephact"));
    command.args(args);
    if let Some(home) = home {
        command.env("HOME", home);
    } else {
        command.env_remove("HOME");
    }
    command.output().expect("ephact should start")
}

#[test]
fn settings_show_uses_config_path_under_home_directory() {
    let home = TempDir::new().expect("temporary home directory");
    let output = ephact_command(Some(home.path()), &["settings", "show"]);
    let stdout = String::from_utf8_lossy(&output.stdout);

    assert!(output.status.success());
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
    let output = ephact_command(None, &["settings", "show"]);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert_eq!(output.status.code(), Some(1));
    assert!(stderr.contains("settings path could not be resolved: HOME is not set"));
}
