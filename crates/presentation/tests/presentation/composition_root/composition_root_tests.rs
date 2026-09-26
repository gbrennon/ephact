#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ephact::{
        application::{
            dtos::{
                requests::RunActionRequest,
                responses::{ExecuteActionResponse, ShowProjectBrandingInfoResponse},
            },
            ports::inbound::{RunActionPort, ShowProjectBrandingInfoPort},
        },
        domain::Settings,
        infrastructure::di::AppContainer,
        presentation::{
            cli::run_progress_handler::RunProgressHandler, components::terminal::Terminal,
            composition_root::CompositionRoot,
        },
    };

    use crate::{
        common::fakes::{
            fake_list_actions_port::FakeListActionsPort,
            fake_list_workflows_port::FakeListWorkflowsPort,
            fake_run_all_workflows_port::FakeRunAllWorkflowsPort,
            fake_run_workflow_port::FakeRunWorkflowPort,
        },
        fakes::fake_settings_store::FakeSettingsStore,
    };

    struct FakeShowProjectBrandingInfoPort;

    impl ShowProjectBrandingInfoPort for FakeShowProjectBrandingInfoPort {
        fn execute(
            &self,
        ) -> Result<
            ShowProjectBrandingInfoResponse,
            ephact::application::errors::ShowProjectBrandingInfoError,
        > {
            Ok(ShowProjectBrandingInfoResponse::new(
                "ephact".to_string(),
                "Ephemeral action runner".to_string(),
                "0.1.0".to_string(),
                "shield".to_string(),
            ))
        }
    }

    struct FakeRunActionPort;

    impl RunActionPort for FakeRunActionPort {
        fn execute(
            &self,
            _request: RunActionRequest,
        ) -> Result<ExecuteActionResponse, ephact::application::errors::RunActionError> {
            Ok(ExecuteActionResponse::note("action completed"))
        }
    }
    struct TestTerminal;

    impl Terminal for TestTerminal {
        fn dimensions(&self) -> (usize, usize) {
            (100, 40)
        }

        fn write_text(&self, _text: &str) -> std::io::Result<()> {
            Ok(())
        }

        fn read_line(&self) -> std::io::Result<String> {
            Ok(String::new())
        }
    }

    fn make_container() -> AppContainer {
        AppContainer::new((
            Box::new(FakeShowProjectBrandingInfoPort),
            Box::new(FakeRunAllWorkflowsPort::new(true)),
            Box::new(FakeRunWorkflowPort::new(true)),
            Box::new(|_| Box::new(FakeRunActionPort)),
            Box::new(FakeListWorkflowsPort::new()),
            Box::new(FakeListActionsPort::new()),
        ))
    }

    #[test]
    fn compose_creates_app_with_container_services() {
        let container = make_container();

        let _app = CompositionRoot::compose(container);
    }

    #[test]
    fn compose_with_tui_progress_and_settings_configures_application() {
        let container = make_container();
        let (_, stream) = RunProgressHandler::with_tui_stream(false);
        let store = Arc::new(FakeSettingsStore::new(Settings::default()));
        let app = CompositionRoot::compose_with_tui_progress_and_settings(
            container,
            stream,
            Settings::default(),
            store,
        );
        let terminal = TestTerminal;

        let output = app
            .run_with_terminal(["ephact", "--help"], &terminal)
            .expect("help should render");

        assert!(output.contains("Usage:"));
    }
}
