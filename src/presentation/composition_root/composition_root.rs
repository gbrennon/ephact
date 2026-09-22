use super::application::Application;
use crate::{
    infrastructure::di::AppContainer,
    presentation::cli::{Cli, cli::CliDependencies},
};

pub struct CompositionRoot;

impl CompositionRoot {
    pub fn compose(container: AppContainer) -> Application {
        Self::compose_internal(container, None)
    }

    pub fn compose_with_tui_progress(
        container: AppContainer,
        progress_stream: crate::presentation::cli::TuiProgressStream,
    ) -> Application {
        Self::compose_internal(container, Some(progress_stream))
    }
    pub fn compose_with_tui_progress_and_settings(
        container: AppContainer,
        progress_stream: crate::presentation::cli::TuiProgressStream,
        settings: crate::domain::Settings,
        store: std::sync::Arc<dyn crate::application::ports::outbound::SettingsStorePort>,
    ) -> Application {
        Self::compose_internal(container, Some(progress_stream)).with_settings(settings, store)
    }

    fn compose_internal(
        container: AppContainer,
        progress_stream: Option<crate::presentation::cli::TuiProgressStream>,
    ) -> Application {
        let failure_log_error_store = container.failure_log_error_store();
        let failure_log_path_store = container.failure_log_path_store();
        let (
            show_project_branding_info_port,
            run_all_workflows_port,
            run_workflow_port,
            _run_action_port,
            discover_run_inputs_port,
            list_workflows_port,
            list_actions_port,
        ) = container.into_parts();
        let dependencies = CliDependencies::new(
            (
                run_workflow_port,
                run_all_workflows_port,
                discover_run_inputs_port,
            ),
            (
                list_workflows_port,
                list_actions_port,
                show_project_branding_info_port,
            ),
        );
        let stores = crate::infrastructure::logging::FailureLogStores::from_stores(
            failure_log_error_store,
            failure_log_path_store,
        );
        let cli = match progress_stream {
            Some(stream) => {
                Cli::new_with_failure_stores_and_progress_stream(dependencies, stores, stream)
            }
            None => Cli::new_with_failure_stores(dependencies, stores),
        };
        Application::new(cli)
    }
}
