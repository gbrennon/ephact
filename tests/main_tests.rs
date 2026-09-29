#[path = "common/main_process.rs"]
mod cli_process;

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::cli_process::run_with_home;

    #[test]
    fn help_command_exits_successfully() {
        let home = TempDir::new().expect("temporary home directory");
        let output = run_with_home(home.path(), &["--help"]);

        assert_eq!(output.status.code(), Some(0));
        assert!(String::from_utf8_lossy(&output.stdout).contains("Usage: ephact"));
    }

    #[test]
    fn invalid_command_exits_with_an_error() {
        let home = TempDir::new().expect("temporary home directory");
        let output = run_with_home(home.path(), &["--invalid"]);

        assert_eq!(output.status.code(), Some(1));
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("unexpected argument '--invalid'")
        );
    }
}
