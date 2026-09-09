#[cfg(test)]
mod tests {
    use std::{cell::RefCell, error::Error, time::Duration};

    use ephact::{
        application::{
            dtos::{
                JobSummary, ListWorkflowsRequest, ListWorkflowsResponse, RunAllWorkflowsRequest,
                RunSummary, RunWorkflowRequest, WorkflowListItem,
            },
            ports::inbound::{ListWorkflowsPort, RunAllWorkflowsPort, RunWorkflowPort},
        },
        presentation::{
            cli::{parse_run_test_args, run_handler::RunHandler},
            components::terminal::{SystemTerminal, Terminal},
        },
    };

    use crate::common::fakes::{
        fake_list_workflows_port::FakeListWorkflowsPort,
        stub_run_all_workflows_port::StubRunAllWorkflowsPort,
        stub_run_workflow_port::StubRunWorkflowPort,
    };

    fn summary(success: bool) -> RunSummary {
        RunSummary::new(
            "test",
            vec![JobSummary::new("job", None, vec![], success)],
            success,
            Duration::ZERO,
        )
    }

    struct RecordingRunWorkflowPort {
        requests: RefCell<Vec<RunWorkflowRequest>>,
    }

    impl RecordingRunWorkflowPort {
        fn new() -> Self {
            Self {
                requests: RefCell::new(Vec::new()),
            }
        }

        fn requests(&self) -> Vec<RunWorkflowRequest> {
            self.requests.borrow().clone()
        }
    }

    impl RunWorkflowPort for RecordingRunWorkflowPort {
        fn execute(&self, request: RunWorkflowRequest) -> Result<RunSummary, Box<dyn Error>> {
            self.requests.borrow_mut().push(request);
            Ok(summary(true))
        }
    }

    struct UnusedRunAllWorkflowsPort;

    impl RunAllWorkflowsPort for UnusedRunAllWorkflowsPort {
        fn execute(&self, _request: RunAllWorkflowsRequest) -> Result<RunSummary, Box<dyn Error>> {
            Err("all workflows should not run interactively".into())
        }
    }

    struct WorkflowListPortFake {
        response: ListWorkflowsResponse,
    }

    impl WorkflowListPortFake {
        fn new(workflows: Vec<WorkflowListItem>) -> Self {
            Self {
                response: ListWorkflowsResponse::new(workflows),
            }
        }
    }

    impl ListWorkflowsPort for WorkflowListPortFake {
        fn execute(
            &self,
            _request: ListWorkflowsRequest,
        ) -> Result<ListWorkflowsResponse, Box<dyn Error>> {
            Ok(self.response.clone())
        }
    }

    struct ScriptedTerminal {
        reads: RefCell<Vec<String>>,
        writes: RefCell<String>,
    }

    impl ScriptedTerminal {
        fn new(reads: Vec<&str>) -> Self {
            Self {
                reads: RefCell::new(reads.into_iter().map(str::to_string).rev().collect()),
                writes: RefCell::new(String::new()),
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

        fn write_text(&self, text: &str) -> std::io::Result<()> {
            self.writes.borrow_mut().push_str(text);
            Ok(())
        }

        fn read_line(&self) -> std::io::Result<String> {
            Ok(self.reads.borrow_mut().pop().unwrap_or_default())
        }
    }

    struct RejectingReadTerminal;

    impl Terminal for RejectingReadTerminal {
        fn dimensions(&self) -> (usize, usize) {
            (100, 40)
        }

        fn write_text(&self, _text: &str) -> std::io::Result<()> {
            Ok(())
        }

        fn read_line(&self) -> std::io::Result<String> {
            Err(std::io::Error::other("stdin should not be read"))
        }
    }

    #[test]
    fn handle_success() {
        let args = parse_run_test_args(&[]);
        let wf_port = StubRunWorkflowPort {
            result: Ok(summary(true)),
        };
        let all_wf_port = StubRunAllWorkflowsPort {
            result: Ok(summary(true)),
        };
        let terminal = SystemTerminal;
        let list_port = FakeListWorkflowsPort::new();
        assert!(RunHandler::handle(args, &wf_port, &all_wf_port, &list_port, &terminal).is_ok());
    }

    #[test]
    fn handle_propagates_workflow_failure() {
        let args = parse_run_test_args(&[]);
        let wf_port = StubRunWorkflowPort {
            result: Ok(summary(false)),
        };
        let all_wf_port = StubRunAllWorkflowsPort {
            result: Ok(summary(false)),
        };
        let terminal = SystemTerminal;
        let list_port = FakeListWorkflowsPort::new();
        let err =
            RunHandler::handle(args, &wf_port, &all_wf_port, &list_port, &terminal).unwrap_err();
        assert!(err.to_string().contains("workflow failed"));
    }

    #[test]
    fn handle_propagates_port_error() {
        let args = parse_run_test_args(&[]);
        let wf_port = StubRunWorkflowPort {
            result: Err("port failure".into()),
        };
        let all_wf_port = StubRunAllWorkflowsPort {
            result: Err("port failure".into()),
        };
        let terminal = SystemTerminal;
        let list_port = FakeListWorkflowsPort::new();
        let err =
            RunHandler::handle(args, &wf_port, &all_wf_port, &list_port, &terminal).unwrap_err();
        assert!(err.to_string().contains("port failure"));
    }

    #[test]
    fn non_interactive_execution_does_not_read_stdin() {
        let args = parse_run_test_args(&[]);
        let wf_port = StubRunWorkflowPort {
            result: Ok(summary(true)),
        };
        let all_wf_port = StubRunAllWorkflowsPort {
            result: Ok(summary(true)),
        };
        let list_port = FakeListWorkflowsPort::new();
        let terminal = RejectingReadTerminal;

        let result =
            RunHandler::handle_with_output(args, &wf_port, &all_wf_port, &list_port, &terminal);

        assert!(result.is_ok());
    }

    #[test]
    fn interactive_selection_supplies_the_pull_request_event() {
        let args = parse_run_test_args(&["--interactive"]);
        let run_port = RecordingRunWorkflowPort::new();
        let all_run_port = UnusedRunAllWorkflowsPort;
        let list_port = WorkflowListPortFake::new(vec![WorkflowListItem::new(
            Some("CI".into()),
            Some("ci.yml".into()),
            vec!["pull_request".into()],
        )]);
        let terminal = ScriptedTerminal::new(vec!["1\n", "\n"]);

        RunHandler::handle_with_output(args, &run_port, &all_run_port, &list_port, &terminal)
            .unwrap();

        let requests = run_port.requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(
            requests[0]
                .config()
                .workflow()
                .map(|workflow| workflow.as_str()),
            Some("CI")
        );
        assert_eq!(
            requests[0].config().event().map(|event| event.as_str()),
            Some("pull_request")
        );
    }

    #[test]
    fn interactive_literal_input_is_forwarded() {
        let args = parse_run_test_args(&["--interactive"]);
        let run_port = RecordingRunWorkflowPort::new();
        let all_run_port = UnusedRunAllWorkflowsPort;
        let list_port = WorkflowListPortFake::new(vec![WorkflowListItem::new(
            Some("CI".into()),
            Some("ci.yml".into()),
            vec!["pull_request".into()],
        )]);
        let terminal = ScriptedTerminal::new(vec!["1\n", "environment=staging\n", "\n"]);

        RunHandler::handle_with_output(args, &run_port, &all_run_port, &list_port, &terminal)
            .unwrap();

        let requests = run_port.requests();
        assert_eq!(requests[0].config().inputs()[0].key(), "environment");
        assert_eq!(requests[0].config().inputs()[0].value(), "staging");
    }

    #[test]
    fn interactive_environment_input_is_resolved_and_forwarded() {
        unsafe {
            std::env::set_var("EPHACT_INTERACTIVE_INPUT_TEST", "production");
        }
        let args = parse_run_test_args(&["--interactive"]);
        let run_port = RecordingRunWorkflowPort::new();
        let all_run_port = UnusedRunAllWorkflowsPort;
        let list_port = WorkflowListPortFake::new(vec![WorkflowListItem::new(
            Some("CI".into()),
            Some("ci.yml".into()),
            vec!["pull_request".into()],
        )]);
        let terminal = ScriptedTerminal::new(vec![
            "1\n",
            "environment=env:EPHACT_INTERACTIVE_INPUT_TEST\n",
            "\n",
        ]);

        RunHandler::handle_with_output(args, &run_port, &all_run_port, &list_port, &terminal)
            .unwrap();

        let requests = run_port.requests();
        assert_eq!(requests[0].config().inputs()[0].key(), "environment");
        assert_eq!(requests[0].config().inputs()[0].value(), "production");
    }

    #[test]
    fn interactive_missing_environment_input_fails_clearly() {
        unsafe {
            std::env::remove_var("EPHACT_INTERACTIVE_INPUT_MISSING_TEST");
        }
        let args = parse_run_test_args(&["--interactive"]);
        let run_port = RecordingRunWorkflowPort::new();
        let all_run_port = UnusedRunAllWorkflowsPort;
        let list_port = WorkflowListPortFake::new(vec![WorkflowListItem::new(
            Some("CI".into()),
            Some("ci.yml".into()),
            vec!["pull_request".into()],
        )]);
        let terminal = ScriptedTerminal::new(vec![
            "1\n",
            "environment=env:EPHACT_INTERACTIVE_INPUT_MISSING_TEST\n",
        ]);

        let error =
            RunHandler::handle_with_output(args, &run_port, &all_run_port, &list_port, &terminal)
                .unwrap_err();

        assert!(
            error
                .to_string()
                .contains("EPHACT_INTERACTIVE_INPUT_MISSING_TEST")
        );
        assert!(run_port.requests().is_empty());
    }

    #[test]
    fn interactive_lists_only_pull_request_workflows() {
        let args = parse_run_test_args(&["--interactive"]);
        let run_port = RecordingRunWorkflowPort::new();
        let all_run_port = UnusedRunAllWorkflowsPort;
        let list_port = WorkflowListPortFake::new(vec![
            WorkflowListItem::new(
                Some("Merge".into()),
                Some("merge.yml".into()),
                vec!["merge_group".into()],
            ),
            WorkflowListItem::new(
                Some("CI".into()),
                Some("ci.yml".into()),
                vec!["pull_request".into()],
            ),
        ]);
        let terminal = ScriptedTerminal::new(vec!["1\n", "\n"]);

        RunHandler::handle_with_output(args, &run_port, &all_run_port, &list_port, &terminal)
            .unwrap();

        assert!(terminal.written_text().contains("CI"));
        assert!(!terminal.written_text().contains("Merge"));
        assert_eq!(
            run_port.requests()[0]
                .config()
                .workflow()
                .map(|workflow| workflow.as_str()),
            Some("CI")
        );
    }
}
