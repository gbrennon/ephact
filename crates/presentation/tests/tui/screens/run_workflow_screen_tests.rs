use std::time::Duration;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ephact::{
    application::dtos::responses::{
        JobSummaryResponse, RunInputDeclarationResponse, RunInputSourceResponse,
        RunSummaryResponse, StepSummaryDetails, StepSummaryResponse, StepSummaryResponseInput,
        WorkflowListItemResponse,
    },
    domain::value_objects::StepType,
    presentation::tui::{components::ScreenFrame, screens::RunWorkflowScreen},
};
use ratatui::{Terminal, backend::TestBackend};

const RUN_CONFIGURATION_KEY: KeyCode = KeyCode::Char('r');

fn select_input(screen: &mut RunWorkflowScreen) {
    screen.handle_configuration_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
}

fn open_input_editor(screen: &mut RunWorkflowScreen) {
    screen.handle_configuration_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
}

fn dismiss_configuration_error(screen: &mut RunWorkflowScreen) {
    screen.handle_configuration_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    screen.handle_configuration_key(KeyEvent::new(RUN_CONFIGURATION_KEY, KeyModifiers::NONE));
}

fn workflows() -> Vec<WorkflowListItemResponse> {
    vec![
        WorkflowListItemResponse::new(Some("Build".into()), None, vec![]),
        WorkflowListItemResponse::new(Some("CI".into()), None, vec![]),
    ]
}

fn rendered_text(screen: &RunWorkflowScreen) -> String {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).expect("test terminal");

    terminal
        .draw(|frame| {
            let area = ScreenFrame::render(frame, "test quote");
            screen.render(frame, area);
        })
        .expect("render screen");

    terminal
        .backend()
        .buffer()
        .content()
        .iter()
        .map(|cell| cell.symbol())
        .collect()
}

fn rendered_text_with_size(screen: &RunWorkflowScreen, width: u16, height: u16) -> String {
    let backend = TestBackend::new(width, height);
    let mut terminal = Terminal::new(backend).expect("test terminal");

    terminal
        .draw(|frame| {
            let area = ScreenFrame::render(frame, "test quote");
            screen.render(frame, area);
        })
        .expect("render screen");

    terminal
        .backend()
        .buffer()
        .content()
        .iter()
        .map(|cell| cell.symbol())
        .collect()
}

fn failed_summary() -> RunSummaryResponse {
    let step = StepSummaryResponse::new(StepSummaryResponseInput::new(
        "compile",
        StepType::Run,
        StepSummaryDetails::new(
            Some(1),
            false,
            Duration::from_secs(1),
            "",
            "compiler failed",
        ),
    ));
    let job = JobSummaryResponse::new("build", Some("build".into()), vec![step], false);
    RunSummaryResponse::new("CI", vec![job], false, Duration::from_secs(1))
}

fn rendered_line_containing(screen: &RunWorkflowScreen, text: &str) -> String {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).expect("test terminal");

    terminal
        .draw(|frame| {
            let area = ScreenFrame::render(frame, "test quote");
            screen.render(frame, area);
        })
        .expect("render screen");

    for line in terminal.backend().buffer().content().chunks(80) {
        let rendered: String = line.iter().map(|cell| cell.symbol()).collect();
        if rendered.contains(text) {
            return rendered;
        }
    }
    panic!("rendered text not found");
}

fn configuration_screen() -> RunWorkflowScreen {
    let mut screen = RunWorkflowScreen::new(workflows());
    screen.begin_configuration(
        vec!["push".to_string()],
        vec![RunInputDeclarationResponse::new(
            "environment",
            RunInputSourceResponse::Workflow,
            None,
            true,
            None,
        )],
    );
    screen
}

fn configuration_picker_shows_keybinds() {
    let screen = configuration_screen();
    let picker = rendered_text(&screen);

    assert!(picker.contains("Up/Down/j/k: Move"));
    assert!(!picker.contains("Enter: Edit"));
    assert!(picker.contains("r: Run"));
    assert!(picker.contains("Esc/Bksp: Back"));
}

fn input_picker_shows_keybinds() {
    let mut screen = configuration_screen();
    select_input(&mut screen);

    let input_picker = rendered_text(&screen);
    assert!(input_picker.contains("Enter: Edit"));
}

fn input_editor_shows_keybinds() {
    let mut screen = configuration_screen();
    select_input(&mut screen);
    open_input_editor(&mut screen);

    let editing = rendered_text(&screen);
    assert!(editing.contains("Type: Edit"));
    assert!(editing.contains("Input: environment (required) = _"));
    assert!(editing.contains("Enter/Esc: Finish"));
    assert!(editing.contains("Bksp: Delete"));
}

fn configuration_error_shows_keybinds() {
    let mut screen = configuration_screen();
    select_input(&mut screen);
    open_input_editor(&mut screen);
    dismiss_configuration_error(&mut screen);

    let error = rendered_text(&screen);
    assert!(error.contains("Enter/Esc/Bksp: Dismiss"));
}

#[test]
fn picker_footer_is_inside_the_screen_border() {
    let screen = RunWorkflowScreen::new(workflows());

    let line = rendered_line_containing(&screen, "Up/Down/j/k: Move");
    let footer_start = line.find("Up/Down/j/k: Move").expect("footer text");

    assert!(line[..footer_start].ends_with("│ "));
    assert!(line[footer_start..].contains('│'));
}

#[test]
fn picker_renders_all_picker_keybinds() {
    let screen = RunWorkflowScreen::new(workflows());
    let text = rendered_text(&screen);

    assert!(text.contains("Up/Down/j/k: Move"));
    assert!(text.contains("Enter: Configure"));
    assert!(!text.contains("d: Details"));
    assert!(text.contains("Esc/Bksp: Back"));
    assert!(text.contains("q: Quit"));
}

#[test]
fn selecting_next_stops_at_last_workflow() {
    let mut screen = RunWorkflowScreen::new(workflows());

    screen.select_next();
    screen.select_next();

    assert_eq!(screen.selected_index(), 1);
}

#[test]
fn selected_workflow_name_returns_highlighted_workflow() {
    let mut screen = RunWorkflowScreen::new(workflows());

    screen.select_next();

    assert_eq!(screen.selected_workflow_name(), Some("CI"));
}

#[test]
fn picker_renders_workflow_names() {
    let screen = RunWorkflowScreen::new(workflows());

    let text = rendered_text(&screen);

    assert!(text.contains("Build"));
    assert!(text.contains("CI"));
}

#[test]
fn empty_picker_renders_repository_message() {
    let screen = RunWorkflowScreen::new(Vec::new());

    let text = rendered_text(&screen);

    assert!(text.contains("No workflows found in repository"));
}

#[test]
fn recording_outcome_renders_run_summary_with_status() {
    let mut screen = RunWorkflowScreen::new(workflows());
    let job = JobSummaryResponse::new("build", Some("build".into()), vec![], true);
    let summary = RunSummaryResponse::new("CI", vec![job], true, Duration::from_secs(1));

    screen.record_outcome(summary);

    let text = rendered_text(&screen);
    assert!(text.contains("Run Summary"));
    assert!(text.contains("SUCCESS"));
}

#[test]
fn running_state_renders_streamed_progress_lines() {
    let mut screen = RunWorkflowScreen::new(workflows());
    screen.start_run();
    screen.record_progress("Step 'compile': running...".to_string());

    let text = rendered_text(&screen);

    assert!(text.contains("Step 'compile': running..."));
}

#[test]
fn failed_summary_offers_details_without_indenting_jobs() {
    let mut screen = RunWorkflowScreen::new(workflows());
    screen.record_outcome(failed_summary());

    let text = rendered_text(&screen);

    assert!(text.contains("d: Details"));
    assert!(text.contains("build: FAILED"));
    assert!(!text.contains("  build: FAILED"));
}

#[test]
fn successful_summary_offers_details() {
    let mut screen = RunWorkflowScreen::new(workflows());
    screen.record_outcome(RunSummaryResponse::new(
        "CI",
        vec![],
        true,
        Duration::from_secs(1),
    ));

    let text = rendered_text(&screen);

    assert!(text.contains("d: Details"));
}

#[test]
fn failure_details_render_step_output() {
    let mut screen = RunWorkflowScreen::new(workflows());
    screen.record_outcome(failed_summary());

    assert!(screen.open_details());
    let text = rendered_text(&screen);

    assert!(text.contains("Run Details"));
    assert!(text.contains("Step: compile [FAILED]"));
    assert!(text.contains("Exit code: 1"));
    assert!(text.contains("stderr: compiler failed"));
}
#[test]
fn failed_summary_renders_failure_diagnostics_path() {
    let mut screen = RunWorkflowScreen::new(workflows());
    screen.record_outcome_with_failure_log_path(
        failed_summary(),
        Some("/tmp/ephact/repository/run.log".into()),
    );

    let text = rendered_text(&screen);

    assert!(text.contains("Failure diagnostics: /tmp/ephact/repository/run.log"));
}

#[test]
fn successful_summary_does_not_render_failure_diagnostics_path() {
    let mut screen = RunWorkflowScreen::new(workflows());
    screen.record_outcome_with_failure_log_path(
        RunSummaryResponse::new("CI", vec![], true, Duration::from_secs(1)),
        Some("/tmp/ephact/repository/run.log".into()),
    );

    let text = rendered_text(&screen);

    assert!(!text.contains("Failure diagnostics:"));
}

#[test]
fn summary_scrolls_with_navigation_keys() {
    let mut screen = RunWorkflowScreen::new(workflows());
    screen.record_outcome(RunSummaryResponse::new(
        "CI",
        vec![],
        true,
        Duration::from_secs(1),
    ));

    screen.scroll_summary_down();

    assert_eq!(screen.summary_scroll(), 1);
}

#[test]
fn summary_scroll_reaches_jobs_after_wrapping_long_diagnostics_path() {
    let mut screen = RunWorkflowScreen::new(workflows());
    let long_path = format!("/tmp/{}", "diagnostics/very-long-segment/".repeat(8));
    screen.record_outcome_with_failure_log_path(failed_summary(), Some(long_path.into()));

    for _ in 0..20 {
        screen.scroll_summary_down();
    }

    let text = rendered_text_with_size(&screen, 30, 8);

    assert!(text.contains("build: FAILED"), "rendered text: {text:?}");
}

#[test]
fn summary_and_details_render_their_keybinds() {
    let mut screen = RunWorkflowScreen::new(workflows());
    screen.record_outcome(RunSummaryResponse::new(
        "CI",
        vec![],
        true,
        Duration::from_secs(1),
    ));

    let summary = rendered_text(&screen);
    assert!(summary.contains("Up/Down/j/k: Scroll"));
    assert!(summary.contains("d: Details"));
    assert!(summary.contains("Esc/Bksp: Back"));
    assert!(summary.contains("q: Quit"));

    screen.open_details();

    let details = rendered_text(&screen);
    assert!(details.contains("Enter/Space: Fold"));
}

#[test]
fn successful_summary_opens_details() {
    let mut screen = RunWorkflowScreen::new(workflows());
    screen.record_outcome(RunSummaryResponse::new(
        "CI",
        vec![],
        true,
        Duration::from_secs(1),
    ));

    assert!(screen.open_details());
    assert!(screen.showing_details());
}

#[test]
fn event_configuration_selects_event_before_input_configuration() {
    let mut screen = RunWorkflowScreen::new(workflows());
    screen.begin_configuration(vec!["push".to_string(), "schedule".to_string()], vec![]);

    let picker = rendered_text(&screen);
    assert!(picker.contains("Select event"));
    assert!(picker.contains("Enter: Select"));
    assert!(!picker.contains("r: Run"));

    screen.handle_configuration_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    let action = screen.handle_configuration_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert_eq!(
        action,
        ephact::presentation::tui::components::ConfigurationAction::Submit
    );
    assert_eq!(
        screen.take_configuration().expect("selected event").event(),
        "schedule"
    );
}

#[test]
fn configuration_renders_keybinds_for_each_state() {
    configuration_picker_shows_keybinds();
    input_picker_shows_keybinds();
    input_editor_shows_keybinds();
    configuration_error_shows_keybinds();
}

#[test]
fn text_input_cursor_is_after_existing_value() {
    let mut screen = RunWorkflowScreen::new(workflows());
    screen.begin_configuration(
        vec!["push".to_string()],
        vec![RunInputDeclarationResponse::new(
            "rustc-version",
            RunInputSourceResponse::Workflow,
            None,
            false,
            Some("stable".to_string()),
        )],
    );
    screen.handle_configuration_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    screen.handle_configuration_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert!(rendered_text(&screen).contains("Input: rustc-version = stable_"));
}

#[test]
fn text_input_uses_the_selected_custom_marker() {
    let mut screen = RunWorkflowScreen::new(workflows());
    screen.begin_configuration_with_marker(
        vec!["push".to_string()],
        vec![RunInputDeclarationResponse::new(
            "rustc-version",
            RunInputSourceResponse::Workflow,
            None,
            false,
            Some("stable".to_string()),
        )],
        &ephact::domain::value_objects::Marker::custom_text("❯"),
    );
    screen.handle_configuration_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    screen.handle_configuration_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert!(rendered_text(&screen).contains("Input: rustc-version = stable❯"));
}

#[test]
fn boolean_inputs_render_and_toggle_boolean_alternatives() {
    let mut screen = RunWorkflowScreen::new(workflows());
    screen.begin_configuration(
        vec!["push".to_string()],
        vec![
            RunInputDeclarationResponse::new(
                "include-sysroot",
                RunInputSourceResponse::Workflow,
                None,
                false,
                Some("false".to_string()),
            )
            .with_type(None),
        ],
    );
    screen.handle_configuration_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    screen.handle_configuration_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    let editing = rendered_text(&screen);
    assert!(editing.contains("[false] true"));
    assert!(editing.contains("Left/h: False"));
    assert!(editing.contains("Right/l: True"));

    screen.handle_configuration_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    assert!(rendered_text(&screen).contains("false [true]"));
    screen.handle_configuration_key(KeyEvent::new(KeyCode::Char('h'), KeyModifiers::NONE));
    assert!(rendered_text(&screen).contains("[false] true"));
    screen.handle_configuration_key(KeyEvent::new(KeyCode::Char('l'), KeyModifiers::NONE));
    screen.handle_configuration_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
    screen.handle_configuration_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE));

    let values = screen.take_configuration().expect("boolean configuration");
    assert_eq!(
        values.inputs(),
        &[("include-sysroot".to_string(), "true".to_string())]
    );
}

#[test]
fn configuration_collects_event_and_required_input() {
    let mut screen = RunWorkflowScreen::new(workflows());
    screen.begin_configuration(
        vec!["push".to_string(), "schedule".to_string()],
        vec![RunInputDeclarationResponse::new(
            "environment",
            RunInputSourceResponse::Workflow,
            None,
            true,
            None,
        )],
    );

    screen.handle_configuration_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    screen.handle_configuration_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    screen.handle_configuration_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    screen.handle_configuration_key(KeyEvent::new(KeyCode::Char('p'), KeyModifiers::NONE));
    screen.handle_configuration_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE));
    screen.handle_configuration_key(KeyEvent::new(KeyCode::Char('o'), KeyModifiers::NONE));
    screen.handle_configuration_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert_eq!(
        screen.handle_configuration_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE)),
        ephact::presentation::tui::components::ConfigurationAction::Submit
    );
    let configuration = screen.take_configuration().expect("configuration");
    assert_eq!(configuration.event(), "schedule");
    assert_eq!(
        configuration.inputs(),
        &[("environment".to_string(), "pro".to_string())]
    );
}

#[test]
fn configuration_rejects_missing_required_input() {
    let mut screen = RunWorkflowScreen::new(workflows());
    screen.begin_configuration(
        vec!["push".to_string()],
        vec![RunInputDeclarationResponse::new(
            "environment",
            RunInputSourceResponse::Workflow,
            None,
            true,
            None,
        )],
    );
    screen.handle_configuration_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));

    assert_eq!(
        screen.handle_configuration_key(KeyEvent::new(KeyCode::Char('r'), KeyModifiers::NONE)),
        ephact::presentation::tui::components::ConfigurationAction::Continue
    );
    assert!(screen.configuration_error().is_some());
}

#[test]
fn running_state_renders_only_cancellation_keybinds() {
    let mut screen = RunWorkflowScreen::new(workflows());
    screen.start_run();

    let text = rendered_text(&screen);
    assert!(text.contains("Esc/Bksp: Cancel"));
    assert!(!text.contains("q: Quit"));
}

#[test]
fn failure_details_scroll_in_both_directions() {
    let mut screen = RunWorkflowScreen::new(workflows());
    screen.record_outcome(failed_summary());
    screen.open_details();

    screen.scroll_details_down();
    assert_eq!(screen.details_scroll(), 1);

    screen.scroll_details_up();
    screen.scroll_details_up();
    assert_eq!(screen.details_scroll(), 0);
}

#[test]
fn reset_returns_screen_to_picker() {
    let mut screen = RunWorkflowScreen::new(workflows());
    screen.record_outcome(RunSummaryResponse::new(
        "CI",
        vec![],
        true,
        Duration::from_secs(1),
    ));

    screen.reset();

    assert!(screen.outcome().is_none());
}

#[test]
fn running_state_renders_cancellation_prompt() {
    let mut screen = RunWorkflowScreen::new(workflows());

    screen.start_run();

    let text = rendered_text(&screen);
    assert!(screen.is_running());
    assert!(text.contains("Workflow is running..."));
    assert!(text.contains("Esc/Bksp: Cancel"));
}
