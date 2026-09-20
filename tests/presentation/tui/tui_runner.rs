#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::time::Duration;

    use ephact::application::dtos::responses::RunSummaryResponse;
    use ephact::presentation::tui::TuiRunner;

    use crate::common::fakes::fake_list_actions_port::FakeListActionsPort;
    use crate::common::fakes::fake_list_workflows_port::FakeListWorkflowsPort;
    use crate::fakes::recording_run_workflow_port::RecordingRunWorkflowPort;

    fn ci_summary() -> RunSummaryResponse {
        RunSummaryResponse::new("CI", vec![], true, Duration::from_secs(1))
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
