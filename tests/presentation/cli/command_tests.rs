#[cfg(test)]
mod tests {
    use ephact::{
        application::{
            dtos::responses::ShowProjectBrandingInfoResponse,
            ports::inbound::ShowProjectBrandingInfoPort,
        },
        presentation::cli::{Cli, cli::CliDependencies},
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

    fn make_cli() -> Cli {
        Cli::new(CliDependencies::new(
            (
                Box::new(FakeRunWorkflowPort::new(true)),
                Box::new(FakeRunAllWorkflowsPort::new(true)),
                Box::new(FakeDiscoverRunInputsPort::new()),
            ),
            (
                Box::new(FakeListWorkflowsPort::new()),
                Box::new(FakeListActionsPort::new()),
                Box::new(FakeShowProjectBrandingInfoPort),
            ),
        ))
    }

    #[test]
    fn run_subcommand_dispatches_to_run_handler() {
        let cli = make_cli();

        let result = cli.run(["ephact", "run"]);

        assert!(result.is_ok());
    }

    #[test]
    fn list_workflows_subcommand_dispatches_to_list_workflows_handler() {
        let cli = make_cli();

        let result = cli.run(["ephact", "list-workflows"]);

        assert!(result.is_ok());
    }

    #[test]
    fn list_actions_subcommand_dispatches_to_list_actions_handler() {
        let cli = make_cli();

        let result = cli.run(["ephact", "list-actions"]);

        assert!(result.is_ok());
    }
}
