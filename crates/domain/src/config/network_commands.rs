//! Command fragments used by [`StepNetworkPolicy`](crate::value_objects::StepNetworkPolicy).
//!
//! Keep these lists centralized so policy checks and their future callers use the
//! same command vocabulary.

/// Commands that may access package registries during a workflow step.
pub const PACKAGE_MANAGER_COMMANDS: &[&str] = &[
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

/// Commands that are blocked when a step has no network access.
pub const NETWORK_COMMANDS: &[&str] = &["curl ", "wget ", "git clone "];

/// Commands that mutate a remote environment and are always blocked.
pub const REMOTE_MUTATION_COMMANDS: &[&str] = &[
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

/// Returns whether `script` contains any configured command fragment.
pub fn contains_command(script: &str, commands: &[&str]) -> bool {
    commands.iter().any(|command| script.contains(command))
}
