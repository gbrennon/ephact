use super::{FailureLogScenario, RunTask};
use crate::application::dtos::responses::RunSummaryResponse;

#[tokio::test]
async fn failed_tui_run_consumes_only_the_active_run_log_path() {
    let stale_run_id = "run-stale";
    let active_run_id = "run-active";
    let mut scenario = FailureLogScenario::new(&[stale_run_id, active_run_id]);
    let handle = tokio::spawn(async {
        Err::<RunSummaryResponse, String>("workflow could not be read".to_string())
    });
    let mut run_task = Some(RunTask {
        handle,
        run_id: active_run_id.to_string(),
    });
    let mut cancelled = false;
    tokio::task::yield_now().await;

    let result = scenario
        .runner
        .process_finished_run(&mut scenario.app, &mut run_task, &mut cancelled)
        .await;

    assert!(result.is_ok());
    assert!(scenario.path(active_run_id).exists());
    assert!(scenario.path(stale_run_id).exists());
    assert!(scenario.handler.path_store().take(active_run_id).is_none());
    assert!(scenario.handler.path_store().take(stale_run_id).is_some());
    scenario.cleanup();
}
