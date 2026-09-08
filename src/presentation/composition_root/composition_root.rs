use super::application::Application;
use crate::{infrastructure::di::AppContainer, presentation::cli::Cli};

pub struct CompositionRoot;

impl CompositionRoot {
    pub fn compose(container: AppContainer) -> Application {
        Application {
            cli: Cli::new(
                container.run_workflow_port,
                container.run_all_workflows_port,
                container.list_workflows_port,
                container.list_actions_port,
                container.show_project_branding_info_port,
            ),
        }
    }
}
