use std::collections::HashMap;

use crate::dtos::responses::RunnerContextResponse;

/// Configuration options for creating and running a container.
#[derive(Debug, Clone, Default)]
pub struct ContainerConfigOptions {
    platform: Option<String>,
    env: HashMap<String, String>,
    binds: Vec<String>,
    workdir: Option<String>,
    cmd: Option<Vec<String>>,
    entrypoint: Option<Vec<String>>,
    network: Option<String>,
    name: Option<String>,
    runner_context: RunnerContextResponse,
}
/// Owned parts of a container configuration.
pub type ContainerConfigOptionsParts = (
    Option<String>,
    HashMap<String, String>,
    Vec<String>,
    Option<String>,
    Option<Vec<String>>,
    Option<Vec<String>>,
    Option<String>,
    Option<String>,
    RunnerContextResponse,
);

impl ContainerConfigOptions {
    /// Sets the target platform.
    pub fn with_platform(self, platform: Option<String>) -> Self {
        Self { platform, ..self }
    }

    /// Sets the environment entries.
    pub fn with_env(self, env: HashMap<String, String>) -> Self {
        Self { env, ..self }
    }

    /// Sets the bind mount entries.
    pub fn with_binds(self, binds: Vec<String>) -> Self {
        Self { binds, ..self }
    }

    /// Sets the working directory.
    pub fn with_workdir(self, workdir: Option<String>) -> Self {
        Self { workdir, ..self }
    }

    /// Sets the command arguments.
    pub fn with_cmd(self, cmd: Option<Vec<String>>) -> Self {
        Self { cmd, ..self }
    }

    /// Sets the entrypoint arguments.
    pub fn with_entrypoint(self, entrypoint: Option<Vec<String>>) -> Self {
        Self { entrypoint, ..self }
    }

    /// Sets the network.
    pub fn with_network(self, network: Option<String>) -> Self {
        Self { network, ..self }
    }

    /// Sets the container name.
    pub fn with_name(self, name: Option<String>) -> Self {
        Self { name, ..self }
    }

    /// Sets the runner context.
    pub fn with_runner_context(self, runner_context: RunnerContextResponse) -> Self {
        Self {
            runner_context,
            ..self
        }
    }

    /// Consumes the options and returns their owned configuration parts.
    pub fn into_parts(self) -> ContainerConfigOptionsParts {
        (
            self.platform,
            self.env,
            self.binds,
            self.workdir,
            self.cmd,
            self.entrypoint,
            self.network,
            self.name,
            self.runner_context,
        )
    }
}
