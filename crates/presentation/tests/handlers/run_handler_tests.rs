mod tests {
    use std::{env, time::Duration};

    use ephact::{
        application::dtos::responses::{
            JobSummaryResponse, RunSummaryResponse, StepSummaryDetails, StepSummaryResponse,
            StepSummaryResponseInput,
        },
        domain::value_objects::StepType,
        presentation::handlers::RunHandler,
    };

    use crate::{
        common::fakes::stub_run_workflow_port::StubRunWorkflowPort,
        fakes::recording_run_workflow_port::RecordingRunWorkflowPort,
    };

    fn run_summary(success: bool) -> RunSummaryResponse {
        RunSummaryResponse::new("CI", vec![], success, Duration::from_secs(1))
    }

    fn current_repository_path() -> std::path::PathBuf {
        env::current_dir()
            .expect("current directory")
            .canonicalize()
            .expect("canonical repository path")
    }

    #[tokio::test]
    async fn handle_returns_summary_from_port() {
        let summary = run_summary(true);
        let port = RecordingRunWorkflowPort::new(summary.clone());

        let response = RunHandler::handle(
            &port,
            env::current_dir().expect("current directory"),
            Some("CI".to_string()),
        )
        .await
        .expect("repository should be valid");

        assert_eq!(response, summary);
    }

    #[tokio::test]
    async fn handle_executes_port_with_selected_workflow_and_safe_defaults() {
        let port = RecordingRunWorkflowPort::new(run_summary(true));

        RunHandler::handle(
            &port,
            env::current_dir().expect("current directory"),
            Some("CI".to_string()),
        )
        .await
        .expect("repository should be valid");

        let request = port.recorded_request().expect("recorded request");
        assert_eq!(request.repository_path(), current_repository_path());
        assert_eq!(request.workflow(), Some("CI"));
        assert!(request.job().is_none());
        assert!(request.event().is_none());
        assert!(request.inputs().is_empty());
        assert!(request.secrets().is_empty());
        assert!(!request.all_workflows());
        assert!(!request.allow_repo_writes());
        assert!(!request.allow_real_container());
        assert!(!request.allow_real_fetcher());
        assert!(!request.allow_network());
        assert!(!request.run_id().is_empty());
    }

    #[tokio::test]
    async fn handle_with_event_and_inputs_forwards_tui_configuration() {
        let port = RecordingRunWorkflowPort::new(run_summary(true));

        RunHandler::handle_with_event_and_inputs(
            &port,
            env::current_dir().expect("current directory"),
            Some("CI".to_string()),
            Some("push".to_string()),
            vec![("environment".to_string(), "staging".to_string())],
        )
        .await
        .expect("workflow should run");

        let request = port.recorded_request().expect("recorded request");
        assert_eq!(request.event(), Some("push"));
        assert_eq!(
            request.inputs(),
            &[("environment".to_string(), "staging".to_string())]
        );
    }

    #[tokio::test]
    async fn handle_executes_port_without_workflow_when_selection_is_unnamed() {
        let port = RecordingRunWorkflowPort::new(run_summary(true));

        RunHandler::handle(&port, env::current_dir().expect("current directory"), None)
            .await
            .expect("repository should be valid");

        let request = port.recorded_request().expect("recorded request");
        assert!(request.workflow().is_none());
        assert!(!request.all_workflows());
    }

    #[tokio::test]
    async fn handle_returns_error_on_invalid_repo_path() {
        let port = RecordingRunWorkflowPort::new(run_summary(true));

        let result = RunHandler::handle(&port, "/definitely/not/a/repository".into(), None).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn handle_returns_error_when_port_fails() {
        let port = StubRunWorkflowPort {
            result: Err("workflow failed".to_string()),
        };

        let result = RunHandler::handle(
            &port,
            env::current_dir().expect("current directory"),
            Some("CI".to_string()),
        )
        .await;

        assert!(result.is_err());
    }

    fn job(job_id: &str, name: Option<&str>, success: bool) -> JobSummaryResponse {
        JobSummaryResponse::new(job_id, name.map(Into::into), vec![], success)
    }

    fn summary(
        success: bool,
        jobs: Vec<JobSummaryResponse>,
        duration: Duration,
    ) -> RunSummaryResponse {
        RunSummaryResponse::new("test", jobs, success, duration)
    }

    #[test]
    fn render_starts_with_summary_heading() {
        let rendered = Rendered::of(&summary(true, vec![], Duration::from_secs(5)));
        assert_eq!(rendered.line(0), "Summary");
    }

    #[test]
    fn render_lists_every_job_with_its_status_in_the_summary() {
        let rendered = Rendered::of(&summary(
            false,
            vec![
                job("build", Some("Build"), true),
                job("validate", None, false),
            ],
            Duration::ZERO,
        ));
        assert_eq!(rendered.line(0), "Summary");
        assert_eq!(rendered.line(1), "Workflow: test");
        assert_eq!(rendered.line(2), "  [ok] build (Build)");
        assert_eq!(rendered.line(3), "  [failed] validate");
    }

    #[test]
    fn render_includes_workflow_and_every_step_status() {
        let summary = RunSummaryResponse::new(
            "Build",
            vec![JobSummaryResponse::new(
                "compile",
                Some("Compile".to_string()),
                vec![
                    StepSummaryResponse::new(StepSummaryResponseInput::new(
                        "Checkout",
                        StepType::Run,
                        StepSummaryDetails::new(
                            Some(0),
                            false,
                            Duration::ZERO,
                            String::new(),
                            String::new(),
                        ),
                    )),
                    StepSummaryResponse::new(StepSummaryResponseInput::new(
                        "Build",
                        StepType::Run,
                        StepSummaryDetails::new(
                            Some(1),
                            false,
                            Duration::ZERO,
                            String::new(),
                            String::new(),
                        ),
                    )),
                ],
                false,
            )],
            false,
            Duration::ZERO,
        );

        let rendered = RunHandler::render(&summary);

        assert!(rendered.contains("Workflow: Build"));
        assert!(rendered.contains("[ok] Step 'Checkout'"));
        assert!(rendered.contains("[failed] Step 'Build'"));
    }

    #[test]
    fn render_reports_failed_step_status_without_output_details() {
        let summary = RunSummaryResponse::new(
            "test",
            vec![JobSummaryResponse::new(
                "lint",
                Some("Lint".to_string()),
                vec![StepSummaryResponse::new(StepSummaryResponseInput::new(
                    "Clippy",
                    StepType::Run,
                    StepSummaryDetails::new(
                        Some(101),
                        false,
                        Duration::ZERO,
                        String::new(),
                        "clippy failed".to_string(),
                    ),
                ))],
                false,
            )],
            false,
            Duration::from_secs(2),
        );

        let rendered = RunHandler::render(&summary);

        assert!(rendered.contains("[failed] Step 'Clippy'"));
        assert!(!rendered.contains("clippy failed"));
    }

    #[test]
    fn render_includes_summary_heading_without_jobs() {
        let rendered = Rendered::of(&summary(true, vec![], Duration::ZERO));
        assert_eq!(rendered.lines().count(), 2);
    }

    struct Rendered {
        text: String,
    }

    impl Rendered {
        fn of(summary: &RunSummaryResponse) -> Self {
            Self {
                text: RunHandler::render(summary),
            }
        }

        fn line(&self, index: usize) -> &str {
            self.text.lines().nth(index).unwrap()
        }

        fn lines(&self) -> std::str::Lines<'_> {
            self.text.lines()
        }
    }
}
