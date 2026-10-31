use std::collections::HashMap;

use crate::{entities::Step, value_objects::ContainerSpecification};

#[derive(Debug, Clone, PartialEq, Default)]
pub(super) struct JobRuntime {
    /// The type of machine to run the job on (e.g. `ubuntu-latest`).
    runs_on: Option<String>,

    /// The sequence of steps to execute.
    steps: Vec<Step>,

    /// Environment variables scoped to this job.
    env: HashMap<String, String>,

    /// Container to run the job inside.
    container: Option<ContainerSpecification>,

    /// Service containers to run alongside the job.
    services: HashMap<String, ContainerSpecification>,
}

impl JobRuntime {
    pub(super) fn new(runs_on: Option<String>, steps: Vec<Step>) -> Self {
        Self {
            runs_on,
            steps,
            ..Self::default()
        }
    }

    pub(super) fn with_env(mut self, env: HashMap<String, String>) -> Self {
        self.env = env;
        self
    }

    pub(super) fn with_container(mut self, container: Option<ContainerSpecification>) -> Self {
        self.container = container;
        self
    }

    pub(super) fn with_services(
        mut self,
        services: HashMap<String, ContainerSpecification>,
    ) -> Self {
        self.services = services;
        self
    }

    pub(super) fn runs_on(&self) -> Option<&str> {
        self.runs_on.as_deref()
    }

    pub(super) fn steps(&self) -> &[Step] {
        &self.steps
    }

    pub(super) fn env(&self) -> &HashMap<String, String> {
        &self.env
    }

    pub(super) fn container(&self) -> Option<&ContainerSpecification> {
        self.container.as_ref()
    }

    pub(super) fn services(&self) -> &HashMap<String, ContainerSpecification> {
        &self.services
    }
}
