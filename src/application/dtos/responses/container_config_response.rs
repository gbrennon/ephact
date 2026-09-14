use crate::application::dtos::responses::RunnerContextResponse;
use std::collections::HashMap;

/// Configuration for creating a container: image, environment, mounts, and the
/// runner context exposed to steps. Consumed by the outbound
/// [`ContainerRuntimePort`](crate::application::ports::outbound::ContainerRuntimePort).
#[derive(Debug, Clone)]
pub struct ContainerConfigResponse {
    image: String,
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
}

impl ContainerConfigResponse {
    pub fn new(image: impl Into<String>, options: ContainerConfigOptions) -> Self {
        Self {
            image: image.into(),
            platform: options.platform,
            env: options.env,
            binds: options.binds,
            workdir: options.workdir,
            cmd: options.cmd,
            entrypoint: options.entrypoint,
            network: options.network,
            name: options.name,
            runner_context: options.runner_context,
        }
    }

    pub fn image(&self) -> &str {
        &self.image
    }

    pub fn platform(&self) -> Option<&str> {
        self.platform.as_deref()
    }

    pub fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub fn binds(&self) -> &[String] {
        &self.binds
    }

    pub fn workdir(&self) -> Option<&str> {
        self.workdir.as_deref()
    }

    pub fn cmd(&self) -> Option<&[String]> {
        self.cmd.as_deref()
    }

    pub fn entrypoint(&self) -> Option<&[String]> {
        self.entrypoint.as_deref()
    }

    pub fn network(&self) -> Option<&str> {
        self.network.as_deref()
    }

    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn runner_context(&self) -> &RunnerContextResponse {
        &self.runner_context
    }
}
