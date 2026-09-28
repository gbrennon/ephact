use crate::config::network_commands::{
    NETWORK_COMMANDS, PACKAGE_MANAGER_COMMANDS, REMOTE_MUTATION_COMMANDS, contains_command,
};

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
        match (
            contains_package_manager_command(&self.script),
            contains_http_request(&self.script),
            contains_network_command(&self.script),
        ) {
            (true, _, _) => None,
            (false, true, _) => {
                Some("network access is disabled; the step would send an HTTP request")
            }
            (false, false, true) => {
                Some("network access is disabled; the step would use a network command")
            }
            (false, false, false) => None,
        }
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
    contains_command(script, PACKAGE_MANAGER_COMMANDS)
}

fn contains_network_command(script: &str) -> bool {
    contains_command(script, NETWORK_COMMANDS)
}

fn contains_remote_mutation(script: &str) -> bool {
    contains_command(script, REMOTE_MUTATION_COMMANDS)
}
