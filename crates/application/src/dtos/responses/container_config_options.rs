use std::collections::HashMap;

use crate::dtos::responses::RunnerContextResponse;

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
    pub fn with_platform(mut self, platform: Option<String>) -> Self {
        self.platform = platform;
        self
    }

    pub fn with_env(mut self, env: HashMap<String, String>) -> Self {
        self.env = env;
        self
    }

    pub fn with_binds(mut self, binds: Vec<String>) -> Self {
        self.binds = binds;
        self
    }

    pub fn with_workdir(mut self, workdir: Option<String>) -> Self {
        self.workdir = workdir;
        self
    }

    pub fn with_cmd(mut self, cmd: Option<Vec<String>>) -> Self {
        self.cmd = cmd;
        self
    }

    pub fn with_entrypoint(mut self, entrypoint: Option<Vec<String>>) -> Self {
        self.entrypoint = entrypoint;
        self
    }

    pub fn with_network(mut self, network: Option<String>) -> Self {
        self.network = network;
        self
    }

    pub fn with_name(mut self, name: Option<String>) -> Self {
        self.name = name;
        self
    }

    pub fn with_runner_context(mut self, runner_context: RunnerContextResponse) -> Self {
        self.runner_context = runner_context;
        self
    }

    pub(crate) fn into_parts(self) -> ContainerConfigOptionsParts {
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
