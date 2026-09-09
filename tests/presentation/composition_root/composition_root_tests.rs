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
            Ok(ShowProjectBrandingInfoResponse {
                name: "ephact".to_string(),
                description: "Ephemeral action runner".to_string(),
                version: "0.1.0".to_string(),
                emblem: "shield".to_string(),
            })
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
        let container = AppContainer {
            show_project_branding_info_port: Box::new(FakeShowProjectBrandingInfoPort),
            run_workflow_port: Box::new(FakeRunWorkflowPort::new(true)),
            run_all_workflows_port: Box::new(FakeRunAllWorkflowsPort::new(true)),
            run_action_port: Box::new(FakeRunActionPort),
            list_workflows_port: Box::new(FakeListWorkflowsPort::new()),
            list_actions_port: Box::new(FakeListActionsPort::new()),
        };

        let _app = CompositionRoot::compose(container);
    }
}
