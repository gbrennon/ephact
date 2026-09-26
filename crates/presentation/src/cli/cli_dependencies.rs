use crate::application::ports::{
    inbound::{
        list_actions_port::ListActionsPort, list_workflows_port::ListWorkflowsPort,
        run_all_workflows_port::RunAllWorkflowsPort, run_workflow_port::RunWorkflowPort,
        show_project_branding_info_port::ShowProjectBrandingInfoPort,
    },
    outbound::RunInputsDiscovererPort,
};

pub type CliRunDependencies = (
    Box<dyn RunWorkflowPort>,
    Box<dyn RunAllWorkflowsPort>,
    Box<dyn RunInputsDiscovererPort>,
);
pub type CliListDependencies = (
    Box<dyn ListWorkflowsPort>,
    Box<dyn ListActionsPort>,
    Box<dyn ShowProjectBrandingInfoPort>,
);
pub type CliParts = (
    Box<dyn RunWorkflowPort>,
    Box<dyn RunAllWorkflowsPort>,
    Box<dyn RunInputsDiscovererPort>,
    Box<dyn ListWorkflowsPort>,
    Box<dyn ListActionsPort>,
    Box<dyn ShowProjectBrandingInfoPort>,
);

pub struct CliDependencies {
    run_dependencies: CliRunDependencies,
    list_dependencies: CliListDependencies,
}

impl CliDependencies {
    pub fn new(
        run_dependencies: CliRunDependencies,
        list_dependencies: CliListDependencies,
    ) -> Self {
        Self {
            run_dependencies,
            list_dependencies,
        }
    }

    pub(crate) fn into_parts(self) -> CliParts {
        let (run_workflow_port, run_all_workflows_port, discover_run_inputs_port) =
            self.run_dependencies;
        let (list_workflows_port, list_actions_port, show_project_branding_info_port) =
            self.list_dependencies;
        (
            run_workflow_port,
            run_all_workflows_port,
            discover_run_inputs_port,
            list_workflows_port,
            list_actions_port,
            show_project_branding_info_port,
        )
    }
}
