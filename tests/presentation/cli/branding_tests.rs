#[cfg(test)]
mod tests {
    use std::error::Error;

    use ephact::application::dtos::responses::ShowProjectBrandingInfoResponse;
    use ephact::application::ports::inbound::ShowProjectBrandingInfoPort;
    use ephact::presentation::cli::Cli;

    use crate::common::fakes::{
        fake_list_actions_port::FakeListActionsPort,
        fake_list_workflows_port::FakeListWorkflowsPort,
        fake_run_all_workflows_port::FakeRunAllWorkflowsPort,
        fake_run_workflow_port::FakeRunWorkflowPort,
    };
    use crate::fakes::fake_discover_run_inputs_port::FakeDiscoverRunInputsPort;

    struct FailingBrandingPort;

    impl ShowProjectBrandingInfoPort for FailingBrandingPort {
        fn execute(&self) -> Result<ShowProjectBrandingInfoResponse, Box<dyn Error>> {
            Err(std::io::Error::other("branding unavailable").into())
        }
    }

    #[test]
    fn run_reports_branding_failure_before_cli_parse_failure() {
        let cli = Cli::new(
            Box::new(FakeRunWorkflowPort::new(true)),
            Box::new(FakeRunAllWorkflowsPort::new(true)),
            Box::new(FakeDiscoverRunInputsPort::new()),
            Box::new(FakeListWorkflowsPort::new()),
            Box::new(FakeListActionsPort::new()),
            Box::new(FailingBrandingPort),
        );

        let error = cli.run(["ephact", "--unknown"]).unwrap_err();

        assert!(error.to_string().contains("branding unavailable"));
    }
}
