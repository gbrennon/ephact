use std::{collections::HashMap, path::Path};

use crate::application::ports::outbound::container_port::ContainerPort;

/// Request DTO for the
/// [`RunNodeActionPort`](crate::application::ports::inbound::run_node_action_port::RunNodeActionPort)
/// outbound port.
pub struct RunNodeActionRequest<'a> {
    /// Directory holding the action on the host.
    pub action_dir: &'a Path,
    /// Entry point the action declared.
    pub entry_point: &'a str,
    /// Inputs the action was called with.
    pub inputs: &'a HashMap<String, String>,
    /// Environment the action runs with.
    pub env: &'a HashMap<String, String>,
    /// Container the action runs in.
    pub container: &'a dyn ContainerPort,
}

impl<'a> RunNodeActionRequest<'a> {
    /// Creates a new request.
    pub fn new(
        action_dir: &'a Path,
        entry_point: &'a str,
        inputs: &'a HashMap<String, String>,
        env: &'a HashMap<String, String>,
        container: &'a dyn ContainerPort,
    ) -> Self {
        Self {
            action_dir,
            entry_point,
            inputs,
            env,
            container,
        }
    }

    /// Directory holding the action on the host.
    pub fn action_dir(&self) -> &'a Path {
        self.action_dir
    }

    /// Entry point the action declared.
    pub fn entry_point(&self) -> &'a str {
        self.entry_point
    }

    /// Inputs the action was called with.
    pub fn inputs(&self) -> &'a HashMap<String, String> {
        self.inputs
    }

    /// Environment the action runs with.
    pub fn env(&self) -> &'a HashMap<String, String> {
        self.env
    }

    /// Container the action runs in.
    pub fn container(&self) -> &'a dyn ContainerPort {
        self.container
    }
}
