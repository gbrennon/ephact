use super::{FailureLogScenario, RunTask, render_text};
use crate::application::dtos::responses::RunSummaryResponse;

#[tokio::test]
async fn failed_tui_run_renders_complete_log_path_when_execution_returns_error() {
    let run_id = "run-tui-failure";
    let mut scenario = FailureLogScenario::new(&[run_id]);
    let handle = tokio::spawn(async {
        Err::<RunSummaryResponse, String>("workflow could not be read".to_string())
    });
    let mut run_task = Some(RunTask {
        handle,
        run_id: run_id.to_string(),
    });
    let mut cancelled = false;
    tokio::task::yield_now().await;

    let result = scenario
        .runner
        .process_finished_run(&mut scenario.app, &mut run_task, &mut cancelled)
        .await;

    assert!(result.is_ok());
    let text = render_text(&scenario.app).replace([' ', '│'], "");
    assert!(
        text.contains(&format!(
            "Failurediagnostics:{}",
            scenario.path(run_id).display()
        )),
        "rendered TUI: {text:?}"
    );
    scenario.cleanup();
}
