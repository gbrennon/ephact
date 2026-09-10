use std::collections::HashMap;

use crate::{application::ports::outbound::container_port::ContainerPort, domain::entities::Step};

pub struct RunShellStepRequest<'a> {
    step: &'a Step,

    container: &'a dyn ContainerPort,

    env: &'a HashMap<String, String>,
}

impl<'a> RunShellStepRequest<'a> {
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

    pub fn step(&self) -> &'a Step {
        self.step
    }

    pub fn container(&self) -> &'a dyn ContainerPort {
        self.container
    }

    pub fn env(&self) -> &'a HashMap<String, String> {
        self.env
    }
}
