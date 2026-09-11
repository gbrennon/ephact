#[cfg(test)]
mod tests {
    use ephact::application::dtos::requests::RunActionRequest;
    use ephact::application::dtos::responses::ExecuteActionResponse;
    use ephact::application::dtos::responses::ShowProjectBrandingInfoResponse;
    use ephact::application::ports::inbound::RunActionPort;
    use ephact::application::ports::inbound::ShowProjectBrandingInfoPort;
    use ephact::infrastructure::di::AppContainer;
    use ephact::presentation::composition_root::CompositionRoot;

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
            _request: RunActionRequest<'_>,
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
