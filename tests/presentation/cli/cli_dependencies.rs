#[cfg(test)]
mod tests {
    use ephact::{
        application::{
            dtos::responses::ShowProjectBrandingInfoResponse, errors::ShowProjectBrandingInfoError,
            ports::inbound::ShowProjectBrandingInfoPort,
        },
        presentation::cli::cli::CliDependencies,
    };

    use crate::{
        common::fakes::{
            fake_list_actions_port::FakeListActionsPort,
            fake_list_workflows_port::FakeListWorkflowsPort,
            fake_run_all_workflows_port::FakeRunAllWorkflowsPort,
            fake_run_workflow_port::FakeRunWorkflowPort,
        },
        fakes::fake_discover_run_inputs_port::FakeDiscoverRunInputsPort,
    };

    struct FakeBrandingPort;

    impl ShowProjectBrandingInfoPort for FakeBrandingPort {
        fn execute(&self) -> Result<ShowProjectBrandingInfoResponse, ShowProjectBrandingInfoError> {
            Ok(ShowProjectBrandingInfoResponse::new(
                "ephact".into(),
                "description".into(),
                "version".into(),
                "emblem".into(),
            ))
        }
    }

    #[test]
    fn dependencies_accept_all_cli_ports() {
        let dependencies = CliDependencies::new(
            (
                Box::new(FakeRunWorkflowPort::new(true)),
                Box::new(FakeRunAllWorkflowsPort::new(true)),
                Box::new(FakeDiscoverRunInputsPort::new()),
            ),
            (
                Box::new(FakeListWorkflowsPort::new()),
                Box::new(FakeListActionsPort::new()),
                Box::new(FakeBrandingPort),
            ),
        );

        drop(dependencies);
    }
}
