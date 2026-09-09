use super::application::Application;
use crate::{infrastructure::di::AppContainer, presentation::cli::Cli};

pub struct CompositionRoot;

impl CompositionRoot {
    pub fn compose(container: AppContainer) -> Application {
        let (
            show_project_branding_info_port,
            run_all_workflows_port,
            run_workflow_port,
            _run_action_port,
            list_workflows_port,
            list_actions_port,
        ) = container.into_parts();
        Application::new(Cli::new(
            run_workflow_port,
            run_all_workflows_port,
            list_workflows_port,
            list_actions_port,
            show_project_branding_info_port,
        ))
    }
}
