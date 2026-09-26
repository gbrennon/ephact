#[cfg(test)]
mod tests {

    use ephact::{
        application::{
            dtos::responses::ShowProjectBrandingInfoResponse,
            ports::inbound::ShowProjectBrandingInfoPort,
        },
        presentation::cli::{Cli, CliDependencies},
    };

    use crate::{
        common::fakes::{
            fake_list_actions_port::FakeListActionsPort,
            fake_list_workflows_port::FakeListWorkflowsPort,
            fake_run_all_workflows_port::FakeRunAllWorkflowsPort,
            fake_run_workflow_port::FakeRunWorkflowPort,
        },
        fakes::fake_run_inputs_discoverer_port::FakeRunInputsDiscovererPort,
    };

    struct FailingBrandingPort;

    impl ShowProjectBrandingInfoPort for FailingBrandingPort {
        fn execute(
            &self,
        ) -> Result<
            ShowProjectBrandingInfoResponse,
            ephact::application::errors::ShowProjectBrandingInfoError,
        > {
            Err(
                ephact::application::errors::ShowProjectBrandingInfoError::Store(
                    ephact::application::errors::ProjectBrandingStoreError::Read(
                        "branding unavailable".to_string(),
                    ),
                ),
            )
        }
    }

    #[test]
    fn run_reports_branding_failure_before_cli_parse_failure() {
        let cli = Cli::new(CliDependencies::new(
            (
                Box::new(FakeRunWorkflowPort::new(true)),
                Box::new(FakeRunAllWorkflowsPort::new(true)),
                Box::new(FakeRunInputsDiscovererPort::new()),
            ),
            (
                Box::new(FakeListWorkflowsPort::new()),
                Box::new(FakeListActionsPort::new()),
                Box::new(FailingBrandingPort),
            ),
        ));

        let error = cli.run(["ephact", "--unknown"]).unwrap_err();

        assert!(error.to_string().contains("branding unavailable"));
    }
}
