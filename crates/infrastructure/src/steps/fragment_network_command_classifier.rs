use crate::domain::traits::NetworkCommandClassifier;

/// Classifies a step's script by matching known command and protocol fragments.
///
/// This adapter owns the tool-specific knowledge that the domain must not
/// depend on, satisfying the [`NetworkCommandClassifier`] contract.
pub struct FragmentNetworkCommandClassifier;

impl FragmentNetworkCommandClassifier {
    const PACKAGE_MANAGER_COMMANDS: &'static [&'static str] = &[
        "apt install ",
        "apt-get install ",
        "apt-get update",
        "dnf install ",
        "dnf update",
        "yum install ",
        "yum update",
        "apk add ",
        "apk update",
        "pacman -s",
        "npm install ",
        "npm ci",
        "npm update",
        "yarn install",
        "yarn add ",
        "yarn upgrade",
        "pnpm install",
        "pnpm add ",
        "pnpm update",
        "pip install ",
        "uv install ",
        "uv sync",
        "cargo fetch",
        "cargo vendor",
        "go mod download",
    ];

    const NETWORK_COMMANDS: &'static [&'static str] = &["curl ", "wget ", "git clone "];

    const REMOTE_MUTATION_COMMANDS: &'static [&'static str] = &[
        "git push ",
        "npm publish",
        "yarn publish",
        "pnpm publish",
        "cargo publish",
        "pip upload",
        "twine upload",
        "docker push ",
        "podman push ",
        "-x post",
        "-x put",
        "-x patch",
        "-x delete",
        "--request post",
        "--request put",
        "--request patch",
        "--request delete",
        "curl -d ",
        "curl --data ",
        "curl --data-raw ",
        "curl --data-binary ",
        "curl --upload-file ",
        "wget --post-data ",
        "http post ",
        "http put ",
    ];

    pub fn new() -> Self {
        Self
    }

    fn contains_any(&self, script: &str, commands: &[&str]) -> bool {
        commands.iter().any(|command| script.contains(command))
    }
}

impl Default for FragmentNetworkCommandClassifier {
    fn default() -> Self {
        Self::new()
    }
}

impl NetworkCommandClassifier for FragmentNetworkCommandClassifier {
    fn accesses_package_registry(&self, script: &str) -> bool {
        self.contains_any(script, Self::PACKAGE_MANAGER_COMMANDS)
    }

    fn issues_http_request(&self, script: &str) -> bool {
        script.contains("http://") || script.contains("https://")
    }

    fn uses_network_command(&self, script: &str) -> bool {
        self.contains_any(script, Self::NETWORK_COMMANDS)
    }

    fn mutates_remote_environment(&self, script: &str) -> bool {
        self.contains_any(script, Self::REMOTE_MUTATION_COMMANDS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_manager_commands_are_recognized() {
        let classifier = FragmentNetworkCommandClassifier::new();

        for command in [
            "apt-get install curl",
            "dnf install curl",
            "npm install package",
            "pip install requests",
            "cargo fetch",
            "go mod download",
        ] {
            assert!(classifier.accesses_package_registry(command));
        }
    }

    #[test]
    fn http_urls_are_recognized_as_requests() {
        let classifier = FragmentNetworkCommandClassifier::new();

        assert!(classifier.issues_http_request("echo https://example.com"));
        assert!(classifier.issues_http_request("echo http://example.com"));
        assert!(!classifier.issues_http_request("echo hello"));
    }

    #[test]
    fn network_commands_are_recognized() {
        let classifier = FragmentNetworkCommandClassifier::new();

        assert!(classifier.uses_network_command("wget release.tar.gz"));
        assert!(!classifier.uses_network_command("echo hello"));
    }

    #[test]
    fn remote_mutations_are_recognized() {
        let classifier = FragmentNetworkCommandClassifier::new();

        for command in [
            "git push origin main",
            "npm publish",
            "curl --request post https://example.com",
            "curl --data payload https://example.com",
        ] {
            assert!(classifier.mutates_remote_environment(command));
        }
    }

    #[test]
    fn compiler_flags_are_not_remote_mutations() {
        let classifier = FragmentNetworkCommandClassifier::new();

        assert!(
            !classifier
                .mutates_remote_environment("cargo clippy --all-targets --locked -- -d warnings")
        );
    }
}
