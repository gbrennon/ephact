#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    use crate::common::fakes::{
        fake_list_actions_port::FakeListActionsPort,
        fake_list_workflows_port::FakeListWorkflowsPort,
        fake_run_all_workflows_port::FakeRunAllWorkflowsPort,
        fake_run_workflow_port::FakeRunWorkflowPort,
    };
    use ephact::application::dtos::requests::{ListActionsRequest, RunActionRequest};
    use ephact::application::dtos::responses::{
        ExecuteActionResponse, ListActionsResponse, ShowProjectBrandingInfoResponse,
    };
    use ephact::application::errors::{
        ListActionsError, RunActionError, ShowProjectBrandingInfoError,
    };
    use ephact::application::ports::inbound::ShowProjectBrandingInfoPort;
    use ephact::application::ports::inbound::{ListActionsPort, RunActionPort};
    use ephact::infrastructure::di::AppContainer;
    use ephact::presentation::components::terminal::Terminal;
    use ephact::presentation::composition_root::Application;
    use ephact::presentation::composition_root::CompositionRoot;

    struct FakeShowProjectBrandingInfoPort;
    impl ShowProjectBrandingInfoPort for FakeShowProjectBrandingInfoPort {
        fn execute(&self) -> Result<ShowProjectBrandingInfoResponse, ShowProjectBrandingInfoError> {
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
        ) -> Result<ExecuteActionResponse, RunActionError> {
            Ok(ExecuteActionResponse::note("action completed"))
        }
    }

    struct VisibleListActionsPort;

    impl ListActionsPort for VisibleListActionsPort {
        fn execute(
            &self,
            _request: ListActionsRequest,
        ) -> Result<ListActionsResponse, ListActionsError> {
            Ok(ListActionsResponse::new(vec![
                "actions/checkout@v4".to_string(),
            ]))
        }
    }

    struct SilentTerminal {
        writes: RefCell<String>,
    }

    impl SilentTerminal {
        fn new() -> Self {
            Self {
                writes: RefCell::new(String::new()),
            }
        }
    }

    impl Terminal for SilentTerminal {
        fn dimensions(&self) -> (usize, usize) {
            (100, 40)
        }

        fn write_text(&self, text: &str) -> std::io::Result<()> {
            self.writes.borrow_mut().push_str(text);
            Ok(())
        }

        fn read_line(&self) -> std::io::Result<String> {
            Ok(String::new())
        }
    }

    fn compose_application() -> Application {
        compose_application_with(Box::new(FakeListActionsPort::new()))
    }

    fn compose_application_with(list_actions_port: Box<dyn ListActionsPort>) -> Application {
        CompositionRoot::compose(AppContainer::new((
            Box::new(FakeShowProjectBrandingInfoPort),
            Box::new(FakeRunAllWorkflowsPort::new(true)),
            Box::new(FakeRunWorkflowPort::new(true)),
            Box::new(|_| Box::new(FakeRunActionPort)),
            Box::new(FakeListWorkflowsPort::new()),
            list_actions_port,
        )))
    }

    #[test]
    fn composed_application_runs_help_through_cli_field() {
        let app = compose_application();

        let result = app.run(["ephact"]);

        assert!(result.is_ok());
    }

    #[test]
    fn composed_application_dispatches_list_actions_through_injected_fakes() {
        let app = compose_application();

        let result = app.run(["ephact", "list-actions"]);

        assert!(result.is_ok());
    }
    #[test]
    fn composed_application_renders_injected_actions() {
        let app = compose_application_with(Box::new(VisibleListActionsPort));
        let terminal = SilentTerminal::new();

        let output = app
            .run_with_terminal(["ephact", "list-actions"], &terminal)
            .unwrap();

        assert!(output.contains("checkout@v4"));
    }
}
