mod tests {
    use std::{cell::RefCell, io};

    use ephact::{
        application::dtos::responses::{RunInputDeclarationResponse, RunInputSourceResponse},
        domain::value_objects::WorkflowRunConfig,
        presentation::{cli::InputCollector, components::terminal::Terminal},
    };

    struct ScriptedTerminal {
        reads: RefCell<Vec<String>>,
        writes: RefCell<String>,
        interactive: bool,
    }

    impl ScriptedTerminal {
        fn new(reads: &[&str], interactive: bool) -> Self {
            Self {
                reads: RefCell::new(reads.iter().rev().map(|read| (*read).to_string()).collect()),
                writes: RefCell::new(String::new()),
                interactive,
            }
        }

        fn written_text(&self) -> String {
            self.writes.borrow().clone()
        }
    }

    impl Terminal for ScriptedTerminal {
        fn dimensions(&self) -> (usize, usize) {
            (100, 40)
        }

        fn write_text(&self, text: &str) -> io::Result<()> {
            self.writes.borrow_mut().push_str(text);
            Ok(())
        }

        fn read_line(&self) -> io::Result<String> {
            Ok(self.reads.borrow_mut().pop().unwrap_or_default())
        }

        fn is_interactive(&self) -> bool {
            self.interactive
        }
    }

    #[test]
    fn collect_inputs_forwards_literal_values_and_stops_on_blank_line() {
        let terminal = ScriptedTerminal::new(&["environment=staging", ""], true);

        let config = InputCollector::new(&terminal)
            .collect_inputs(WorkflowRunConfig::new())
            .expect("literal input should be collected");

        assert_eq!(config.inputs().len(), 1);
        assert_eq!(config.inputs()[0].key(), "environment");
        assert_eq!(config.inputs()[0].value(), "staging");
        assert!(terminal.written_text().contains("Additional inputs"));
    }

    #[test]
    fn collect_inputs_resolves_environment_values() {
        let variable = "EPHACT_INPUT_COLLECTOR_TEST";
        unsafe { std::env::set_var(variable, "production") };
        let terminal =
            ScriptedTerminal::new(&["environment=env:EPHACT_INPUT_COLLECTOR_TEST", ""], true);

        let config = InputCollector::new(&terminal)
            .collect_inputs(WorkflowRunConfig::new())
            .expect("environment input should be collected");

        unsafe { std::env::remove_var(variable) };
        assert_eq!(config.inputs()[0].value(), "production");
    }

    #[test]
    fn collect_declared_inputs_prompts_for_required_values() {
        let terminal = ScriptedTerminal::new(&["staging"], true);
        let declarations = vec![RunInputDeclarationResponse::new(
            "environment",
            RunInputSourceResponse::Action("workflow.yml".into()),
            Some("Deployment environment".into()),
            true,
            None,
        )];
        let mut config = WorkflowRunConfig::new();

        InputCollector::new(&terminal)
            .collect_declared_inputs(&mut config, &declarations, true)
            .expect("declared input should be collected");

        assert_eq!(config.inputs()[0].key(), "environment");
        assert_eq!(config.inputs()[0].value(), "staging");
        assert!(terminal.written_text().contains("Deployment environment"));
    }

    #[test]
    fn collect_declared_inputs_reports_missing_required_noninteractive_values() {
        let terminal = ScriptedTerminal::new(&[], false);
        let declarations = vec![RunInputDeclarationResponse::new(
            "token",
            RunInputSourceResponse::Action("workflow.yml".into()),
            None,
            true,
            None,
        )];
        let mut config = WorkflowRunConfig::new();

        let error = InputCollector::new(&terminal)
            .collect_declared_inputs(&mut config, &declarations, false)
            .expect_err("missing required input should fail");

        assert_eq!(error.to_string(), "required inputs missing: token");
    }
}
