use crate::application::ports::inbound::{
    list_actions_port::ListActionsPort, list_workflows_port::ListWorkflowsPort,
    run_action_port::RunActionPort, run_all_workflows_port::RunAllWorkflowsPort,
    run_workflow_port::RunWorkflowPort,
    show_project_branding_info_port::ShowProjectBrandingInfoPort,
};

pub struct AppContainer {
    show_project_branding_info_port: Box<dyn ShowProjectBrandingInfoPort>,
    run_all_workflows_port: Box<dyn RunAllWorkflowsPort>,
    run_workflow_port: Box<dyn RunWorkflowPort>,
    run_action_port: Box<dyn RunActionPort>,
    list_workflows_port: Box<dyn ListWorkflowsPort>,
    list_actions_port: Box<dyn ListActionsPort>,
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
        Self {
            show_project_branding_info_port,
            run_all_workflows_port,
            run_workflow_port,
            run_action_port,
            list_workflows_port,
            list_actions_port,
        }
    }

    pub fn into_parts(
        self,
    ) -> (
        Box<dyn ShowProjectBrandingInfoPort>,
        Box<dyn RunAllWorkflowsPort>,
        Box<dyn RunWorkflowPort>,
        Box<dyn RunActionPort>,
        Box<dyn ListWorkflowsPort>,
        Box<dyn ListActionsPort>,
    ) {
        (
            self.show_project_branding_info_port,
            self.run_all_workflows_port,
            self.run_workflow_port,
            self.run_action_port,
            self.list_workflows_port,
            self.list_actions_port,
        )
    }
}
