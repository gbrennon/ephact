use std::process::Command;

use tempfile::TempDir;

fn ephact_command(home: &std::path::Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_ephact"))
        .args(args)
        .env("HOME", home)
        .output()
        .expect("ephact should start")
}

#[test]
fn help_command_exits_successfully() {
    let home = TempDir::new().expect("temporary home directory");
    let output = ephact_command(home.path(), &["--help"]);

    assert_eq!(output.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&output.stdout).contains("Usage: ephact"));
}

#[test]
fn invalid_command_exits_with_an_error() {
    let home = TempDir::new().expect("temporary home directory");
    let output = ephact_command(home.path(), &["--invalid"]);

    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("unexpected argument '--invalid'"));
}
