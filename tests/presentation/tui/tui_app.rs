use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ephact::presentation::tui::tui_app::{TuiApp, TuiScreen};
#[test]
fn selecting_list_workflows_enters_workflow_screen() {
    let mut app = TuiApp::new(Vec::new());
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));

    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert_eq!(app.screen(), TuiScreen::ListWorkflows);
}

#[test]
fn selecting_list_actions_enters_actions_screen() {
    let mut app = TuiApp::new(Vec::new());
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));

    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert_eq!(app.screen(), TuiScreen::ListActions);
}

#[test]
fn cancel_key_requests_cancellation_without_leaving_run_screen() {
    let mut app = TuiApp::new(vec![
        ephact::application::dtos::responses::WorkflowListItemResponse::new(
            Some("CI".to_string()),
            None,
            Vec::new(),
        ),
    ]);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.take_run_request();
    app.start_run();

    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));

    assert_eq!(app.screen(), TuiScreen::RunWorkflow);
    assert!(app.take_cancel_request());
}

#[test]
fn quit_key_does_not_exit_while_workflow_runs() {
    let mut app = TuiApp::new(vec![
        ephact::application::dtos::responses::WorkflowListItemResponse::new(
            Some("CI".to_string()),
            None,
            Vec::new(),
        ),
    ]);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.take_run_request();
    app.start_run();

    app.handle_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));

    assert_eq!(app.screen(), TuiScreen::RunWorkflow);
}
