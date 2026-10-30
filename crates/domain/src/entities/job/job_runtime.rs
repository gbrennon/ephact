use std::collections::HashMap;

use crate::{entities::Step, value_objects::ContainerSpecification};

#[derive(Debug, Clone, PartialEq, Default)]
pub(super) struct JobRuntime {
    /// The type of machine to run the job on (e.g. `ubuntu-latest`).
    pub(super) runs_on: Option<String>,

    /// The sequence of steps to execute.
    pub(super) steps: Vec<Step>,

    /// Environment variables scoped to this job.
    pub(super) env: HashMap<String, String>,

    /// Container to run the job inside.
    pub(super) container: Option<ContainerSpecification>,

    /// Service containers to run alongside the job.
    pub(super) services: HashMap<String, ContainerSpecification>,
}
