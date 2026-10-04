use crate::{
    application::ports::{inbound::ListWorkflowsPort, outbound::RunInputsDiscovererPort},
    components::terminal::Terminal,
};

pub struct PreflightPorts<'a> {
    discover_run_inputs_port: &'a dyn RunInputsDiscovererPort,
    list_workflows_port: &'a dyn ListWorkflowsPort,
    terminal: &'a dyn Terminal,
}

impl<'a> PreflightPorts<'a> {
    pub fn new(
        discover_run_inputs_port: &'a dyn RunInputsDiscovererPort,
        list_workflows_port: &'a dyn ListWorkflowsPort,
        terminal: &'a dyn Terminal,
    ) -> Self {
        Self {
            discover_run_inputs_port,
            list_workflows_port,
            terminal,
        }
    }

    pub(super) fn discover_run_inputs_port(&self) -> &dyn RunInputsDiscovererPort {
        self.discover_run_inputs_port
    }

    pub(super) fn list_workflows_port(&self) -> &dyn ListWorkflowsPort {
        self.list_workflows_port
    }

    pub(super) fn terminal(&self) -> &dyn Terminal {
        self.terminal
    }
}
