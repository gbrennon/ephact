use std::time::Duration;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ephact::{
    application::dtos::responses::{RunSummaryResponse, WorkflowListItemResponse},
    presentation::tui::{
        screens::ScreenManager,
        tui_app::{TuiApp, TuiScreen},
    },
};
#[test]
fn screen_manager_retains_home_and_workflow_selection() {
    let workflows = vec![
        WorkflowListItemResponse::new(Some("CI".to_string()), None, Vec::new()),
        WorkflowListItemResponse::new(Some("Deploy".to_string()), None, Vec::new()),
    ];
    let mut screens = ScreenManager::new(workflows);

    screens.select_home_next();
    screens.run_workflow_screen_mut().select_next();

    assert_eq!(screens.home_selection(), 1);
    assert_eq!(screens.run_workflow_screen().selected_index(), 1);
}

#[test]
fn tui_app_records_run_outcome_through_screen_manager() {
    let mut app = TuiApp::new(Vec::new());
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    let summary = RunSummaryResponse::new("CI", vec![], true, Duration::from_secs(1));

    app.record_run_outcome(summary.clone());

    assert_eq!(app.run_workflow_screen().outcome(), Some(&summary));
}

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
fn details_key_opens_failed_run_details() {
    let mut app = TuiApp::new(vec![]);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.record_run_outcome(RunSummaryResponse::new(
        "CI",
        vec![],
        false,
        Duration::from_secs(1),
    ));

    app.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));

    assert!(app.run_workflow_screen().showing_details());
}

#[test]
fn tui_configuration_submits_selected_event() {
    let mut app = TuiApp::new(vec![WorkflowListItemResponse::new(
        Some("CI".to_string()),
        None,
        vec!["push".to_string(), "schedule".to_string()],
    )]);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.begin_run_configuration(vec!["push".to_string(), "schedule".to_string()], vec![]);
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE));

    let configuration = app
        .take_configured_run_request()
        .expect("configured run request");

    assert_eq!(configuration.event(), "schedule");
}

#[test]
fn summary_navigation_scrolls_completed_run_summary() {
    let mut app = TuiApp::new(Vec::new());
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.record_run_outcome(RunSummaryResponse::new(
        "CI",
        vec![],
        true,
        Duration::from_secs(1),
    ));

    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));

    assert_eq!(app.run_workflow_screen().summary_scroll(), 1);
}

#[test]
fn details_key_opens_successful_run_details() {
    let mut app = TuiApp::new(vec![]);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.record_run_outcome(RunSummaryResponse::new(
        "CI",
        vec![],
        true,
        Duration::from_secs(1),
    ));

    app.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));

    assert!(app.run_workflow_screen().showing_details());
}

#[test]
fn escape_closes_failed_run_details() {
    let mut app = TuiApp::new(vec![]);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.record_run_outcome(RunSummaryResponse::new(
        "CI",
        vec![],
        false,
        Duration::from_secs(1),
    ));
    app.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));

    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));

    assert!(!app.run_workflow_screen().showing_details());
}

#[test]
fn details_screen_scrolls_with_navigation_keys() {
    let mut app = TuiApp::new(vec![]);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.record_run_outcome(RunSummaryResponse::new(
        "CI",
        vec![],
        false,
        Duration::from_secs(1),
    ));
    app.handle_key(KeyEvent::new(KeyCode::Char('d'), KeyModifiers::NONE));

    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));

    assert_eq!(app.run_workflow_screen().details_scroll(), 1);
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
