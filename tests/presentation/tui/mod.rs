mod event_reader;
mod screens;
mod settings_tests;
mod tui_app;
mod tui_runner;

use std::{env, time::Duration};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ephact::{
    application::dtos::responses::{RunSummaryResponse, WorkflowListItemResponse},
    presentation::{
        handlers::{ListActionsHandler, ListWorkflowsHandler},
        tui::{
            components::{ColorSupport, ScreenFrame, SplashQuotes},
            screen_manager::TuiScreen,
            screens::{HomeScreen, ListActionsScreen, ListWorkflowsScreen, SplashScreen},
            theme::Theme,
            tui_app::TuiApp,
        },
    },
};
use ratatui::{
    Terminal,
    backend::TestBackend,
    buffer::{Buffer, Cell},
    style::{Color, Style},
};

use crate::common::fakes::{
    fake_list_actions_port::FakeListActionsPort, fake_list_workflows_port::FakeListWorkflowsPort,
};

const QUIT_KEY: char = '\x71';
struct TuiRenderAssertions;

impl TuiRenderAssertions {
    fn rendered_buffer(render: impl FnOnce(&mut ratatui::Frame<'_>)) -> Buffer {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).expect("test terminal");
        terminal.draw(render).expect("render screen");
        terminal.backend().buffer().clone()
    }

    fn rendered_screen(
        render: impl FnOnce(&mut ratatui::Frame<'_>, ratatui::layout::Rect),
    ) -> Buffer {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).expect("test terminal");
        terminal
            .draw(|frame| {
                let area = ScreenFrame::render(frame, "test quote");
                render(frame, area);
            })
            .expect("render screen");
        terminal.backend().buffer().clone()
    }

    fn buffer_text(buffer: &Buffer) -> String {
        buffer
            .content()
            .iter()
            .map(Cell::symbol)
            .collect::<String>()
    }

    fn row_text(buffer: &Buffer, row: u16) -> String {
        (0..buffer.area.width)
            .map(|column| buffer[(column, row)].symbol())
            .collect()
    }

    fn row_styles(buffer: &Buffer, row: u16) -> Vec<Style> {
        (0..buffer.area.width)
            .map(|column| buffer[(column, row)].style())
            .collect()
    }

    fn styles_for_label(buffer: &Buffer, label: &str) -> Vec<Style> {
        let row = (0..buffer.area.height)
            .find(|row| Self::row_text(buffer, *row).contains(label))
            .expect("label row");
        let start = Self::label_start_column(buffer, label, row);
        Self::row_styles(buffer, row)[start..start + label.chars().count()].to_vec()
    }

    fn label_start_column(buffer: &Buffer, label: &str, row: u16) -> usize {
        let text = Self::row_text(buffer, row);
        let columns: Vec<char> = text.chars().collect();
        let wanted: Vec<char> = label.chars().collect();
        columns
            .windows(wanted.len())
            .position(|window| window == wanted.as_slice())
            .expect("label on row")
    }
}

#[test]
fn home_frame_title_matches_other_inner_frame_padding() {
    let home = TuiRenderAssertions::rendered_screen(|frame, area| {
        HomeScreen::render(frame, area, 0);
    });
    let actions = TuiRenderAssertions::rendered_screen(|frame, area| {
        ListActionsScreen::new(vec![]).render(frame, area);
    });

    let home_column =
        TuiRenderAssertions::label_start_column(&home, "What would you like to do?", 5);
    let actions_column = TuiRenderAssertions::label_start_column(&actions, "Actions", 5);

    assert_eq!(home_column, actions_column);
}

#[test]
fn home_menu_item_matches_other_inner_frame_padding() {
    let home = TuiRenderAssertions::rendered_screen(|frame, area| {
        HomeScreen::render(frame, area, 0);
    });
    let actions = TuiRenderAssertions::rendered_screen(|frame, area| {
        ListActionsScreen::new(vec!["build".into()]).render(frame, area);
    });

    let home_column = TuiRenderAssertions::label_start_column(&home, "Run workflow", 6);
    let actions_column = TuiRenderAssertions::label_start_column(&actions, "build", 6);

    assert_eq!(home_column, actions_column);
}

#[test]
fn splash_advances_to_home_on_non_q_key() {
    let mut app = TuiApp::new(vec![]);

    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert_eq!(app.screen(), TuiScreen::Home);
}

#[test]
fn q_exits_from_splash() {
    let mut app = TuiApp::new(vec![]);

    app.handle_key(KeyEvent::new(KeyCode::Char(QUIT_KEY), KeyModifiers::NONE));

    assert_eq!(app.screen(), TuiScreen::Exit);
}

#[test]
fn home_navigation_bounds() {
    let mut app = TuiApp::new(vec![]);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    app.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(app.home_selection(), 0);
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(app.home_selection(), 2);
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(app.home_selection(), 3);
}

#[test]
fn home_down_then_enter_transitions_to_list_workflows() {
    let mut app = TuiApp::new(vec![]);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));

    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert_eq!(app.screen(), TuiScreen::ListWorkflows);
}

#[test]
fn list_workflows_esc_returns_to_home() {
    let mut app = TuiApp::new(vec![]);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));

    assert_eq!(app.screen(), TuiScreen::Home);
}

#[test]
fn list_workflows_backspace_returns_to_home() {
    let mut app = TuiApp::new(vec![]);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    app.handle_key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));

    assert_eq!(app.screen(), TuiScreen::Home);
}

#[test]
fn list_workflows_q_exits() {
    let mut app = TuiApp::new(vec![]);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    app.handle_key(KeyEvent::new(KeyCode::Char(QUIT_KEY), KeyModifiers::NONE));

    assert_eq!(app.screen(), TuiScreen::Exit);
}

#[test]
fn list_workflows_selection_bounds() {
    let workflows = vec![
        WorkflowListItemResponse::new(Some("first".into()), None, vec![]),
        WorkflowListItemResponse::new(Some("second".into()), None, vec![]),
    ];
    let mut app = TuiApp::new(workflows);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    app.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(app.selected_workflow_index(), 0);
    app.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(app.selected_workflow_index(), 0);
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(app.selected_workflow_index(), 1);
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(app.selected_workflow_index(), 1);
}

#[test]
fn splash_renders_fallback_emblem_without_true_color() {
    let text = TuiRenderAssertions::buffer_text(&TuiRenderAssertions::rendered_buffer(|frame| {
        SplashScreen::render_with(frame, ColorSupport::Basic, SplashQuotes::select(0));
    }));

    assert!(text.contains("@---o"));
    assert!(text.contains("EPHACT"));
}

#[test]
fn splash_renders_fancy_emblem_with_true_color() {
    let buffer = TuiRenderAssertions::rendered_buffer(|frame| {
        SplashScreen::render_with(frame, ColorSupport::TrueColor, SplashQuotes::select(0));
    });

    let text = TuiRenderAssertions::buffer_text(&buffer);
    let uses_rgb = buffer
        .content()
        .iter()
        .any(|cell| matches!(cell.style().fg, Some(Color::Rgb(_, _, _))));
    assert!(text.contains("@---o"));
    assert!(uses_rgb);
}

#[test]
fn home_renders_initial_menu() {
    let text =
        TuiRenderAssertions::buffer_text(&TuiRenderAssertions::rendered_screen(|frame, area| {
            HomeScreen::render(frame, area, 0)
        }));

    assert!(text.contains("Run workflow"));
    assert!(text.contains("List workflows"));
    assert!(text.contains("List actions"));
}

#[test]
fn home_frame_title_uses_signal_title_style() {
    let buffer = TuiRenderAssertions::rendered_screen(|frame, area| {
        HomeScreen::render(frame, area, 0);
    });

    let title_styles = TuiRenderAssertions::styles_for_label(&buffer, "What would you like to do?");

    assert!(
        title_styles
            .iter()
            .all(|style| style.fg == Some(Theme::SIGNAL_ACCENT))
    );
}

#[test]
fn list_workflows_screen_renders_populated_items() {
    let workflows = vec![
        WorkflowListItemResponse::new(
            Some("CI".into()),
            Some(".github/workflows/ci.yml".into()),
            vec!["push".into(), "pull_request".into()],
        ),
        WorkflowListItemResponse::new(None, None, vec![]),
    ];
    let screen = ListWorkflowsScreen::new(workflows);
    let buffer = TuiRenderAssertions::rendered_screen(|frame, area| screen.render(frame, area));
    let text = TuiRenderAssertions::buffer_text(&buffer);

    assert!(text.contains("CI  (.github/workflows/ci.yml)  [push, pull_request]"));
    assert!(text.contains("Unnamed workflow  (-)  [-]"));
}

#[test]
fn list_workflows_screen_renders_empty_message() {
    let screen = ListWorkflowsScreen::new(vec![]);
    let buffer = TuiRenderAssertions::rendered_screen(|frame, area| screen.render(frame, area));

    assert!(TuiRenderAssertions::buffer_text(&buffer).contains("No workflows found in repository"));
}

#[test]
fn list_workflows_screen_renders_highlight_on_selected_row() {
    let workflows = vec![
        WorkflowListItemResponse::new(Some("first".into()), None, vec![]),
        WorkflowListItemResponse::new(Some("second".into()), None, vec![]),
    ];
    let mut screen = ListWorkflowsScreen::new(workflows);
    screen.select_next();
    let buffer = TuiRenderAssertions::rendered_screen(|frame, area| screen.render(frame, area));

    let selected = TuiRenderAssertions::styles_for_label(&buffer, "second");
    let unselected = TuiRenderAssertions::styles_for_label(&buffer, "first");
    assert!(
        selected
            .iter()
            .all(|style| style.bg == Some(Theme::SIGNAL_ACCENT))
    );
    assert!(
        unselected
            .iter()
            .all(|style| style.bg != Some(Theme::SIGNAL_ACCENT))
    );
}

#[test]
fn home_renders_highlight_on_selected_menu_item() {
    let buffer =
        TuiRenderAssertions::rendered_screen(|frame, area| HomeScreen::render(frame, area, 1));

    let selected = TuiRenderAssertions::styles_for_label(&buffer, "List workflows");
    let unselected = TuiRenderAssertions::styles_for_label(&buffer, "Run workflow");
    assert!(
        selected
            .iter()
            .all(|style| style.bg == Some(Theme::SIGNAL_ACCENT))
    );
    assert!(
        unselected
            .iter()
            .all(|style| style.bg != Some(Theme::SIGNAL_ACCENT))
    );
}

#[test]
fn list_workflows_handler_executes_port_with_current_repo() {
    let workflows = vec![WorkflowListItemResponse::new(
        Some("CI".into()),
        Some("ci.yml".into()),
        vec!["push".into()],
    )];
    let port = FakeListWorkflowsPort::with_workflows(workflows.clone());

    let response =
        ListWorkflowsHandler::handle(&port, env::current_dir().expect("current directory"))
            .expect("current repository should be valid");

    assert_eq!(response.workflows(), workflows);
}

#[test]
fn list_workflows_handler_returns_error_on_invalid_repo_path() {
    let port = FakeListWorkflowsPort::new();

    let result = ListWorkflowsHandler::handle(&port, "/definitely/not/a/repository".into());

    assert!(result.is_err());
}

#[test]
fn home_down_twice_then_enter_transitions_to_list_actions() {
    let mut app = TuiApp::new(vec![]);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));

    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert_eq!(app.screen(), TuiScreen::ListActions);
}

#[test]
fn list_actions_esc_returns_to_home() {
    let mut app = TuiApp::new(vec![]);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));

    assert_eq!(app.screen(), TuiScreen::Home);
}

#[test]
fn list_actions_backspace_returns_to_home() {
    let mut app = TuiApp::new(vec![]);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    app.handle_key(KeyEvent::new(KeyCode::Backspace, KeyModifiers::NONE));

    assert_eq!(app.screen(), TuiScreen::Home);
}

#[test]
fn list_actions_q_exits() {
    let mut app = TuiApp::new(vec![]);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    app.handle_key(KeyEvent::new(KeyCode::Char(QUIT_KEY), KeyModifiers::NONE));

    assert_eq!(app.screen(), TuiScreen::Exit);
}

#[test]
fn list_actions_selection_bounds() {
    let actions = vec![
        "actions/checkout@v4".to_string(),
        "docker://node:20".to_string(),
    ];
    let mut app = TuiApp::new(vec![]).with_actions(actions);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    app.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(app.selected_action_index(), 0);
    app.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
    assert_eq!(app.selected_action_index(), 0);
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(app.selected_action_index(), 1);
    app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
    assert_eq!(app.selected_action_index(), 1);
}

#[test]
fn list_actions_screen_renders_populated_items() {
    let actions = vec![
        "actions/checkout@v4".to_string(),
        "docker://node:20".to_string(),
    ];
    let screen = ListActionsScreen::new(actions);
    let buffer = TuiRenderAssertions::rendered_screen(|frame, area| screen.render(frame, area));
    let text = TuiRenderAssertions::buffer_text(&buffer);

    assert!(text.contains("actions/checkout@v4"));
    assert!(text.contains("docker://node:20"));
}

#[test]
fn list_actions_screen_renders_empty_message() {
    let screen = ListActionsScreen::new(vec![]);
    let buffer = TuiRenderAssertions::rendered_screen(|frame, area| screen.render(frame, area));

    assert!(TuiRenderAssertions::buffer_text(&buffer).contains("No actions found in repository"));
}

#[test]
fn list_actions_screen_renders_highlight_on_selected_row() {
    let actions = vec![
        "actions/checkout@v4".to_string(),
        "docker://node:20".to_string(),
    ];
    let mut screen = ListActionsScreen::new(actions);
    screen.select_next();
    let buffer = TuiRenderAssertions::rendered_screen(|frame, area| screen.render(frame, area));

    let selected = TuiRenderAssertions::styles_for_label(&buffer, "docker://node:20");
    let unselected = TuiRenderAssertions::styles_for_label(&buffer, "actions/checkout@v4");
    assert!(
        selected
            .iter()
            .all(|style| style.bg == Some(Theme::SIGNAL_ACCENT))
    );
    assert!(
        unselected
            .iter()
            .all(|style| style.bg != Some(Theme::SIGNAL_ACCENT))
    );
}

#[test]
fn home_renders_highlight_on_selected_list_actions_item() {
    let buffer = TuiRenderAssertions::rendered_screen(|frame, area| {
        HomeScreen::render(frame, area, HomeScreen::LIST_ACTIONS_INDEX)
    });

    let selected = TuiRenderAssertions::styles_for_label(&buffer, "List actions");
    let unselected = TuiRenderAssertions::styles_for_label(&buffer, "Run workflow");
    assert!(
        selected
            .iter()
            .all(|style| style.bg == Some(Theme::SIGNAL_ACCENT))
    );
    assert!(
        unselected
            .iter()
            .all(|style| style.bg != Some(Theme::SIGNAL_ACCENT))
    );
}

#[test]
fn list_actions_handler_executes_port_with_current_repo() {
    let actions = vec![
        "actions/checkout@v4".to_string(),
        "docker://node:20".to_string(),
    ];
    let port = FakeListActionsPort::with_actions(actions.clone());

    let response =
        ListActionsHandler::handle(&port, env::current_dir().expect("current directory"))
            .expect("current repository should be valid");

    assert_eq!(response.actions(), actions.as_slice());
}

#[test]
fn list_actions_handler_returns_error_on_invalid_repo_path() {
    let port = FakeListActionsPort::new();

    let result = ListActionsHandler::handle(&port, "/definitely/not/a/repository".into());

    assert!(result.is_err());
}

#[test]
fn home_enter_on_run_workflow_transitions_to_run_workflow_screen() {
    let mut app = TuiApp::new(vec![]);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert_eq!(app.screen(), TuiScreen::RunWorkflow);
}

#[test]
fn run_workflow_esc_returns_to_home() {
    let mut app = TuiApp::new(vec![]);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    app.handle_key(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));

    assert_eq!(app.screen(), TuiScreen::Home);
}

#[test]
fn run_workflow_enter_requests_run_when_workflows_present() {
    let workflows = vec![WorkflowListItemResponse::new(
        Some("CI".into()),
        None,
        vec![],
    )];
    let mut app = TuiApp::new(workflows);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert!(app.take_run_request());
}

#[test]
fn run_workflow_enter_ignored_without_workflows() {
    let mut app = TuiApp::new(vec![]);
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));

    assert!(!app.take_run_request());
}

#[test]
fn recording_run_outcome_exposes_summary_on_run_screen() {
    let workflows = vec![WorkflowListItemResponse::new(
        Some("CI".into()),
        None,
        vec![],
    )];
    let mut app = TuiApp::new(workflows);
    let summary = RunSummaryResponse::new("CI", vec![], true, Duration::from_secs(1));

    app.record_run_outcome(summary.clone());

    assert_eq!(app.run_outcome(), Some(&summary));
}
