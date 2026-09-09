#[cfg(test)]
mod tests {
    use ephact::{
        application::{
            dtos::{ExecuteActionResponse, RunActionRequest},
            ports::inbound::{RunActionPort, ShowProjectBrandingInfoPort},
        },
        infrastructure::di::AppContainer,
        presentation::composition_root::{Application, CompositionRoot},
    };

    use crate::common::fakes::{
        fake_list_actions_port::FakeListActionsPort,
        fake_list_workflows_port::FakeListWorkflowsPort,
        fake_run_all_workflows_port::FakeRunAllWorkflowsPort,
        fake_run_workflow_port::FakeRunWorkflowPort,
    };

    struct FakeShowProjectBrandingInfoPort;

    impl ShowProjectBrandingInfoPort for FakeShowProjectBrandingInfoPort {
        fn execute(
            &self,
        ) -> Result<
            ephact::application::dtos::ShowProjectBrandingInfoResponse,
            Box<dyn std::error::Error>,
        > {
            Ok(
                ephact::application::dtos::ShowProjectBrandingInfoResponse::new(
                    "ephact".to_string(),
                    "Ephemeral action runner".to_string(),
                    "0.1.0".to_string(),
                    "shield".to_string(),
                ),
            )
        }
    }

    struct FakeRunActionPort;

    impl RunActionPort for FakeRunActionPort {
        fn execute(
            &self,
            _request: RunActionRequest,
        ) -> Result<ExecuteActionResponse, ephact::domain::errors::StepError> {
            Ok(ExecuteActionResponse::note("action completed"))
        }
    }

    fn compose_application() -> Application {
        CompositionRoot::compose(AppContainer::new(
            Box::new(FakeShowProjectBrandingInfoPort),
            Box::new(FakeRunAllWorkflowsPort::new(true)),
            Box::new(FakeRunWorkflowPort::new(true)),
            Box::new(FakeRunActionPort),
            Box::new(FakeListWorkflowsPort::new()),
            Box::new(FakeListActionsPort::new()),
        ))
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
}
