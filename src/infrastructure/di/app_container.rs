use crate::application::ports::inbound::{
    list_actions_port::ListActionsPort, list_workflows_port::ListWorkflowsPort,
    run_action_port::RunActionPort, run_all_workflows_port::RunAllWorkflowsPort,
    run_workflow_port::RunWorkflowPort,
    show_project_branding_info_port::ShowProjectBrandingInfoPort,
};
use crate::application::ports::outbound::DiscoverRunInputsPort;

pub struct AppContainer {
    show_project_branding_info_port: Box<dyn ShowProjectBrandingInfoPort>,
    run_all_workflows_port: Box<dyn RunAllWorkflowsPort>,
    run_workflow_port: Box<dyn RunWorkflowPort>,
    run_action_port: Box<dyn RunActionPort>,
    discover_run_inputs_port: Box<dyn DiscoverRunInputsPort>,
    list_workflows_port: Box<dyn ListWorkflowsPort>,
    list_actions_port: Box<dyn ListActionsPort>,
}
/// The ports assembled by the application container.
pub type AppContainerParts = (
    Box<dyn ShowProjectBrandingInfoPort>,
    Box<dyn RunAllWorkflowsPort>,
    Box<dyn RunWorkflowPort>,
    Box<dyn RunActionPort>,
    Box<dyn DiscoverRunInputsPort>,
    Box<dyn ListWorkflowsPort>,
    Box<dyn ListActionsPort>,
);
struct EmptyRunInputDiscovery;

impl DiscoverRunInputsPort for EmptyRunInputDiscovery {
    fn execute(
        &self,
        _request: crate::application::dtos::DiscoverRunInputsRequest,
    ) -> Result<Vec<crate::application::dtos::RunInputDeclaration>, Box<dyn std::error::Error>>
    {
        Ok(Vec::new())
    }
}

impl AppContainer {
    pub fn new(
        show_project_branding_info_port: Box<dyn ShowProjectBrandingInfoPort>,
        run_all_workflows_port: Box<dyn RunAllWorkflowsPort>,
        run_workflow_port: Box<dyn RunWorkflowPort>,
        run_action_port: Box<dyn RunActionPort>,
        list_workflows_port: Box<dyn ListWorkflowsPort>,
        list_actions_port: Box<dyn ListActionsPort>,
    ) -> Self {
        Self::new_with_discovery(
            show_project_branding_info_port,
            run_all_workflows_port,
            run_workflow_port,
            run_action_port,
            Box::new(EmptyRunInputDiscovery),
            list_workflows_port,
            list_actions_port,
        )
    }

    pub fn new_with_discovery(
        show_project_branding_info_port: Box<dyn ShowProjectBrandingInfoPort>,
        run_all_workflows_port: Box<dyn RunAllWorkflowsPort>,
        run_workflow_port: Box<dyn RunWorkflowPort>,
        run_action_port: Box<dyn RunActionPort>,
        discover_run_inputs_port: Box<dyn DiscoverRunInputsPort>,
        list_workflows_port: Box<dyn ListWorkflowsPort>,
        list_actions_port: Box<dyn ListActionsPort>,
    ) -> Self {
        Self {
            show_project_branding_info_port,
            run_all_workflows_port,
            run_workflow_port,
            run_action_port,
            discover_run_inputs_port,
            list_workflows_port,
            list_actions_port,
        }
    }

    pub fn into_parts(self) -> AppContainerParts {
        (
            self.show_project_branding_info_port,
            self.run_all_workflows_port,
            self.run_workflow_port,
            self.run_action_port,
            self.discover_run_inputs_port,
            self.list_workflows_port,
            self.list_actions_port,
        )
    }
}
