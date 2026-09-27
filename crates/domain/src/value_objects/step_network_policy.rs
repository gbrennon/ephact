pub struct StepNetworkPolicy {
    script: String,
}

impl StepNetworkPolicy {
    pub fn new(script: &str) -> Self {
        Self {
            script: script.to_ascii_lowercase(),
        }
    }

    pub fn network_access_reason(&self) -> Option<&'static str> {
        if contains_package_manager_command(&self.script) {
            return None;
        }
        if contains_http_request(&self.script) {
            return Some("network access is disabled; the step would send an HTTP request");
        }
        if contains_network_command(&self.script) {
            return Some("network access is disabled; the step would use a network command");
        }
        None
    }

    pub fn network_policy_violation(&self) -> Option<&'static str> {
        contains_remote_mutation(&self.script)
            .then_some("network operation blocked; the step would modify a remote environment")
    }
}

fn contains_http_request(script: &str) -> bool {
    script.contains("http://") || script.contains("https://")
}

fn contains_package_manager_command(script: &str) -> bool {
    [
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
    ]
    .iter()
    .any(|command| script.contains(command))
}

fn contains_network_command(script: &str) -> bool {
    ["curl ", "wget ", "git clone "]
        .iter()
        .any(|command| script.contains(command))
}

fn contains_remote_mutation(script: &str) -> bool {
    [
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
    ]
    .iter()
    .any(|command| script.contains(command))
}

#[cfg(test)]
mod tests {
    use super::StepNetworkPolicy;

    #[test]
    fn allows_package_manager_network_access() {
        for command in [
            "apt-get install curl",
            "npm install requests",
            "pip install requests",
            "cargo fetch",
        ] {
            assert_eq!(
                StepNetworkPolicy::new(command).network_access_reason(),
                None
            );
        }
    }

    #[test]
    fn blocks_http_requests_and_network_commands() {
        assert_eq!(
            StepNetworkPolicy::new("curl https://example.com").network_access_reason(),
            Some("network access is disabled; the step would send an HTTP request")
        );
        assert_eq!(
            StepNetworkPolicy::new("git clone example/repo").network_access_reason(),
            Some("network access is disabled; the step would use a network command")
        );
    }

    #[test]
    fn blocks_remote_mutations() {
        assert_eq!(
            StepNetworkPolicy::new("git push origin main").network_policy_violation(),
            Some("network operation blocked; the step would modify a remote environment")
        );
    }
}
