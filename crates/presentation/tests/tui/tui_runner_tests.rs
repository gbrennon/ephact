#[cfg(test)]
mod tests {
    use std::{
        sync::{Arc, Mutex},
        time::Duration,
    };

    use ephact::{
        application::{
            dtos::{
                requests::RunActionRequest,
                responses::{ExecuteActionResponse, RunSummaryResponse},
            },
            errors::RunActionError,
            ports::inbound::RunActionPort,
        },
        presentation::tui::TuiRunner,
    };

    use crate::{
        common::fakes::{
            fake_list_actions_port::FakeListActionsPort,
            fake_list_workflows_port::FakeListWorkflowsPort,
        },
        fakes::recording_run_workflow_port::RecordingRunWorkflowPort,
    };

    struct RecordingRunActionPort {
        requests: Mutex<Vec<RunActionRequest>>,
    }

    impl RecordingRunActionPort {
        fn new() -> Self {
            Self {
                requests: Mutex::new(Vec::new()),
            }
        }

        fn requests(&self) -> Vec<RunActionRequest> {
            self.requests.lock().expect("request lock").clone()
        }
    }

    impl RunActionPort for RecordingRunActionPort {
        fn execute(
            &self,
            request: RunActionRequest,
        ) -> Result<ExecuteActionResponse, RunActionError> {
            self.requests.lock().expect("request lock").push(request);
            Ok(ExecuteActionResponse::note("action completed".to_owned()))
        }
    }

    fn ci_summary() -> RunSummaryResponse {
        RunSummaryResponse::new("CI".to_owned(), vec![], true, Duration::from_secs(1))
    }

    #[tokio::test]
    async fn run_workflow_executes_run_workflow_port_with_selected_name() {
        let run_port = Arc::new(RecordingRunWorkflowPort::new(ci_summary()));
        let runner = TuiRunner::new(
            Arc::new(FakeListWorkflowsPort::new()),
            Arc::new(FakeListActionsPort::new()),
            run_port.clone(),
        );

        let summary = runner
            .run_workflow(Some("CI".to_string()))
            .await
            .expect("run should succeed");

        assert_eq!(summary, ci_summary());
        assert_eq!(
            run_port
                .recorded_request()
                .expect("recorded request")
                .workflow(),
            Some("CI")
        );
    }

    #[tokio::test]
    async fn run_action_executes_only_selected_action() {
        let action_port = Arc::new(RecordingRunActionPort::new());
        let workflow_port = Arc::new(RecordingRunWorkflowPort::new(ci_summary()));
        let runner = TuiRunner::new(
            Arc::new(FakeListWorkflowsPort::new()),
            Arc::new(FakeListActionsPort::new()),
            workflow_port.clone(),
        )
        .with_action_port(action_port.clone());

        let response = runner
            .run_action(
                "./.github/actions/demo".to_owned(),
                vec![("name".to_owned(), "Ada".to_owned())],
            )
            .await
            .expect("action should succeed");

        assert_eq!(response.stdout(), "action completed");
        let requests = action_port.requests();
        assert_eq!(requests.len(), 1);
        assert_eq!(requests[0].action_ref(), "./.github/actions/demo");
        assert!(requests[0].step().contains("\"name\":\"Ada\""));
        assert!(workflow_port.recorded_request().is_none());
    }

    #[tokio::test]
    async fn run_workflow_does_not_start_nested_runtime() {
        let run_port = Arc::new(RecordingRunWorkflowPort::new(ci_summary()));
        let runner = TuiRunner::new(
            Arc::new(FakeListWorkflowsPort::new()),
            Arc::new(FakeListActionsPort::new()),
            run_port,
        );

        let result = runner.run_workflow(Some("CI".to_string())).await;

        assert!(result.is_ok());
    }
}
