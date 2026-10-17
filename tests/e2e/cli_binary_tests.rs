#[cfg(test)]
mod tests {
    use std::process::{Command, Output};

    use tempfile::TempDir;

    struct CliBinaryScenario {
        repository: TempDir,
    }

    impl CliBinaryScenario {
        fn new() -> Self {
            let repository = TempDir::new().expect("fixture repository should be created");
            let workflows = repository.path().join(".github/workflows");
            std::fs::create_dir_all(&workflows).expect("workflow directory should be created");
            std::fs::write(
                workflows.join("ci.yml"),
                "name: Build\non:\n  pull_request:\njobs:\n  build:\n    runs-on: ubuntu-latest\n    steps:\n      - uses: actions/checkout@v4\n",
            )
            .expect("workflow fixture should be written");
            std::fs::create_dir(repository.path().join(".git"))
                .expect("fixture repository metadata should be created");
            Self { repository }
        }

        fn run(&self, arguments: &[&str]) -> Output {
            Command::new(
                std::env::var_os("CARGO_BIN_EXE_ephact")
                    .expect("Cargo should provide the ephact binary path"),
            )
            .args(arguments)
            .output()
            .expect("ephact should start")
        }

        fn repository_path(&self) -> &std::path::Path {
            self.repository.path()
        }
    }

    #[test]
    fn binary_lists_workflows_from_a_repository() {
        let scenario = CliBinaryScenario::new();
        let output = scenario.run(&[
            "list-workflows",
            &scenario.repository_path().display().to_string(),
        ]);
        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(
            output.status.success(),
            "stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(stdout.contains("Build"), "CLI output: {stdout}");
    }

    #[test]
    fn binary_lists_actions_from_a_repository() {
        let scenario = CliBinaryScenario::new();
        let output = scenario.run(&[
            "list-actions",
            &scenario.repository_path().display().to_string(),
        ]);
        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(
            output.status.success(),
            "stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(stdout.contains("checkout@v4"), "CLI output: {stdout}");
    }

    #[test]
    fn binary_cli_alias_lists_workflows_from_a_repository() {
        let scenario = CliBinaryScenario::new();
        let output = scenario.run(&[
            "cli",
            "list-workflows",
            &scenario.repository_path().display().to_string(),
        ]);
        let stdout = String::from_utf8_lossy(&output.stdout);

        assert!(
            output.status.success(),
            "stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        assert!(stdout.contains("Build"), "CLI output: {stdout}");
    }
}
