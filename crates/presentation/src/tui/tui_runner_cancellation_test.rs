use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::{FailureLogScenario, RunTask};
use crate::application::dtos::responses::RunSummaryResponse;

#[tokio::test]
async fn cancelling_tui_run_removes_its_pending_log_path() {
    let run_id = "run-cancelled";
    let mut scenario = FailureLogScenario::new(&[run_id]);
    scenario
        .app
        .handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    let handle =
        tokio::spawn(async { std::future::pending::<Result<RunSummaryResponse, String>>().await });
    let mut run_task = Some(RunTask {
        handle,
        run_id: run_id.to_string(),
    });
    let mut cancelled = false;

    scenario
        .runner
        .process_cancel_request(&mut scenario.app, &mut run_task, &mut cancelled);

    assert!(scenario.handler.path_store().take(run_id).is_none());
    tokio::task::yield_now().await;
    scenario
        .runner
        .process_finished_run(&mut scenario.app, &mut run_task, &mut cancelled)
        .await
        .expect("cancelled run should be discarded");
    scenario.cleanup();
}
