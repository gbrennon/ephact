use crate::application::dtos::RunnerContext;
use std::collections::HashMap;

/// Configuration for creating a container: image, environment, mounts, and the
/// runner context exposed to steps. Consumed by the outbound
/// [`ContainerRuntimePort`](crate::application::ports::outbound::ContainerRuntimePort).
#[derive(Debug, Clone)]
pub struct ContainerConfig {
    pub image: String,
    pub platform: Option<String>,
    pub env: HashMap<String, String>,
    pub binds: Vec<String>,
    pub workdir: Option<String>,
    pub cmd: Option<Vec<String>>,
    pub entrypoint: Option<Vec<String>>,
    pub network: Option<String>,
    pub name: Option<String>,
    pub runner_context: RunnerContext,
}

impl ContainerConfig {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        image: impl Into<String>,
        platform: Option<String>,
        env: HashMap<String, String>,
        binds: Vec<String>,
        workdir: Option<String>,
        cmd: Option<Vec<String>>,
        entrypoint: Option<Vec<String>>,
        network: Option<String>,
        name: Option<String>,
        runner_context: RunnerContext,
    ) -> Self {
        Self {
            image: image.into(),
            platform,
            env,
            binds,
            workdir,
            cmd,
            entrypoint,
            network,
            name,
            runner_context,
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

    pub fn runner_context(&self) -> &RunnerContext {
        &self.runner_context
    }
}
