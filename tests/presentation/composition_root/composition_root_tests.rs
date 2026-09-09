#[cfg(test)]
mod tests {
    use ephact::{
        application::{
            dtos::{ExecuteActionResponse, RunActionRequest, ShowProjectBrandingInfoResponse},
            ports::inbound::{RunActionPort, ShowProjectBrandingInfoPort},
        },
        infrastructure::di::AppContainer,
        presentation::composition_root::CompositionRoot,
    };

    use crate::common::fakes::{
        fake_list_actions_port::FakeListActionsPort,
        fake_list_workflows_port::FakeListWorkflowsPort,
        fake_run_all_workflows_port::FakeRunAllWorkflowsPort,
        fake_run_workflow_port::FakeRunWorkflowPort,
    };

    struct FakeShowProjectBrandingInfoPort;

    impl ShowProjectBrandingInfoPort for FakeShowProjectBrandingInfoPort {
        fn execute(&self) -> Result<ShowProjectBrandingInfoResponse, Box<dyn std::error::Error>> {
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
        ) -> Result<ExecuteActionResponse, ephact::domain::errors::StepError> {
            Ok(ExecuteActionResponse::note("action completed"))
        }
    }

    #[test]
    fn compose_creates_app_with_container_services() {
        let container = AppContainer::new(
            Box::new(FakeShowProjectBrandingInfoPort),
            Box::new(FakeRunAllWorkflowsPort::new(true)),
            Box::new(FakeRunWorkflowPort::new(true)),
            Box::new(FakeRunActionPort),
            Box::new(FakeListWorkflowsPort::new()),
            Box::new(FakeListActionsPort::new()),
        );

        let _app = CompositionRoot::compose(container);
    }
}
