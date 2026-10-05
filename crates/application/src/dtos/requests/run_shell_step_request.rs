use std::collections::HashMap;

use crate::{domain::entities::Step, ports::outbound::container_port::ContainerPort};

/// Inputs for running a shell step with a container and environment entries.
pub struct RunShellStepRequest<'a> {
    step: &'a Step,
    container: &'a dyn ContainerPort,
    env: &'a HashMap<String, String>,
}

impl<'a> RunShellStepRequest<'a> {
    /// Creates inputs from a step, container, and environment entries.
    pub fn new(
        step: &'a Step,
        container: &'a dyn ContainerPort,
        env: &'a HashMap<String, String>,
    ) -> Self {
        Self {
            step,
            container,
            env,
        }
    }

    /// Returns the step.
    pub fn step(&self) -> &'a Step {
        self.step
    }

    /// Returns the container.
    pub fn container(&self) -> &'a dyn ContainerPort {
        self.container
    }

    /// Returns the environment entries.
    pub fn env(&self) -> &'a HashMap<String, String> {
        self.env
    }
}
