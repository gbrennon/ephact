#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use ephact::{
        application::{
            dtos::responses::ShowProjectBrandingInfoResponse,
            ports::inbound::ShowProjectBrandingInfoPort,
        },
        domain::{InterfaceMode, Settings},
        presentation::{
            cli::{Cli, CliDependencies},
            components::terminal::SystemTerminal,
        },
    };

    use crate::{
        common::fakes::{
            fake_list_actions_port::FakeListActionsPort,
            fake_list_workflows_port::FakeListWorkflowsPort,
            fake_run_all_workflows_port::FakeRunAllWorkflowsPort,
            fake_run_workflow_port::FakeRunWorkflowPort,
        },
        fakes::{
            fake_run_inputs_discoverer_port::FakeRunInputsDiscovererPort,
            fake_settings_store::FakeSettingsStore,
        },
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
                Box::new(FakeRunInputsDiscovererPort::new()),
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
    #[test]
    fn settings_set_persists_one_typed_value() {
        let store = Arc::new(FakeSettingsStore::new(Settings::default()));
        let cli = make_cli().with_settings(Settings::default(), store.clone());
        let terminal = SystemTerminal;

        let output = cli
            .run_with_terminal(
                ["ephact", "settings", "set", "default-interface", "cli"],
                &terminal,
            )
            .expect("settings command should succeed");

        assert!(output.contains("default-interface = cli"));
        assert_eq!(store.writes().len(), 1);
        assert_eq!(store.writes()[0].default_interface(), InterfaceMode::Cli);
    }

    #[test]
    fn invalid_settings_value_is_rejected() {
        let store = Arc::new(FakeSettingsStore::new(Settings::default()));
        let cli = make_cli().with_settings(Settings::default(), store.clone());
        let terminal = SystemTerminal;

        let result = cli.run_with_terminal(
            ["ephact", "settings", "set", "allow-network", "sometimes"],
            &terminal,
        );

        assert!(result.is_err());
        assert!(store.writes().is_empty());
    }
    #[test]
    fn settings_set_persists_boolean_flag() {
        let store = Arc::new(FakeSettingsStore::new(Settings::default()));
        let cli = make_cli().with_settings(Settings::default(), store.clone());
        let terminal = SystemTerminal;

        let output = cli
            .run_with_terminal(
                ["ephact", "settings", "set", "allow-network", "true"],
                &terminal,
            )
            .expect("settings set allow-network should succeed");

        assert!(output.contains("allow-network = true"));
        assert_eq!(store.writes().len(), 1);
        assert!(store.writes()[0].allow_network());
    }

    #[test]
    fn settings_set_sequentially_updates_multiple_settings() {
        let store = Arc::new(FakeSettingsStore::new(Settings::default()));
        let terminal = SystemTerminal;

        make_cli()
            .with_settings(Settings::default(), store.clone())
            .run_with_terminal(
                ["ephact", "settings", "set", "allow-network", "true"],
                &terminal,
            )
            .expect("set allow-network");
        make_cli()
            .with_settings(Settings::default(), store.clone())
            .run_with_terminal(["ephact", "settings", "set", "verbose", "true"], &terminal)
            .expect("set verbose");
        let output = make_cli()
            .with_settings(Settings::default(), store.clone())
            .run_with_terminal(
                ["ephact", "settings", "set", "allow-network", "false"],
                &terminal,
            )
            .expect("set allow-network back to false");

        assert!(output.contains("allow-network = false"));
        assert!(output.contains("verbose = true"));
        assert_eq!(store.writes().len(), 3);
        assert!(!store.writes()[2].allow_network());
        assert!(store.writes()[2].verbose());
    }

    #[test]
    fn settings_reset_restores_default_settings() {
        let modified = Settings::default()
            .with_allow_network(true)
            .with_verbose(true);
        let store = Arc::new(FakeSettingsStore::new(modified.clone()));
        let cli = make_cli().with_settings(modified, store.clone());
        let terminal = SystemTerminal;

        let output = cli
            .run_with_terminal(["ephact", "settings", "reset"], &terminal)
            .expect("settings reset should succeed");

        assert!(output.contains("allow-network = false"));
        assert!(output.contains("verbose = false"));
        assert_eq!(store.writes().len(), 1);
        assert_eq!(store.writes()[0], Settings::default());
    }

    #[test]
    fn persisted_cli_interface_makes_no_subcommand_render_help() {
        let settings = Settings::default().with_default_interface(InterfaceMode::Cli);
        let store = Arc::new(FakeSettingsStore::new(settings.clone()));
        let cli = make_cli().with_settings(settings, store);
        let terminal = SystemTerminal;

        let output = cli
            .run_with_terminal(["ephact"], &terminal)
            .expect("CLI default should render help");

        assert!(output.contains("Usage:"));
    }
}
