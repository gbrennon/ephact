use ephact::presentation::tui::{components::ScreenFrame, screens::ListActionsScreen};
use ratatui::{Terminal, backend::TestBackend};

use crate::common::fakes::fake_list_actions_port::FakeListActionsPort;

#[test]
fn selecting_next_stops_at_last_action() {
    let actions = vec![
        "actions/checkout@v4".to_string(),
        "docker://node:20".to_string(),
    ];
    let mut screen = ListActionsScreen::new(actions);

    screen.select_next();
    screen.select_next();

    assert_eq!(screen.selected_index(), 1);
}

#[test]
fn selecting_previous_decrements_index_and_stops_at_zero() {
    let actions = vec![
        "actions/checkout@v4".to_string(),
        "docker://node:20".to_string(),
    ];
    let mut screen = ListActionsScreen::new(actions);

    screen.select_next();
    assert_eq!(screen.selected_index(), 1);

    screen.select_previous();
    assert_eq!(screen.selected_index(), 0);

    screen.select_previous();
    assert_eq!(screen.selected_index(), 0);
}

#[test]
fn render_empty_screen_shows_repository_message() {
    let screen = ListActionsScreen::new(Vec::new());
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).expect("test terminal");

    terminal
        .draw(|frame| {
            let area = ScreenFrame::render(frame, "test quote");
            screen.render(frame, area);
        })
        .expect("render screen");

    let text = terminal
        .backend()
        .buffer()
        .content()
        .iter()
        .map(|cell| cell.symbol())
        .collect::<String>();
    assert!(text.contains("No actions found in repository"));
    assert!(text.contains("Up/Down/j/k: Move"));
    assert!(text.contains("Esc/Bksp: Back"));
    assert!(text.contains("q: Quit"));
}

#[test]
fn render_populated_screen_shows_action_items() {
    let actions = vec![
        "actions/checkout@v4".to_string(),
        "docker://node:20".to_string(),
    ];
    let screen = ListActionsScreen::new(actions);
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).expect("test terminal");

    terminal
        .draw(|frame| {
            let area = ScreenFrame::render(frame, "test quote");
            screen.render(frame, area);
        })
        .expect("render screen");

    let text = terminal
        .backend()
        .buffer()
        .content()
        .iter()
        .map(|cell| cell.symbol())
        .collect::<String>();
    assert!(text.contains("actions/checkout@v4"));
    assert!(text.contains("docker://node:20"));
}

#[test]
fn from_handler_populates_actions_from_port() {
    let actions = vec![
        "actions/checkout@v4".to_string(),
        "docker://node:20".to_string(),
    ];
    let port = FakeListActionsPort::with_actions(actions.clone());

    let screen =
        ListActionsScreen::from_handler(&port, std::env::current_dir().expect("current directory"))
            .expect("repository should be valid");

    assert_eq!(screen.actions(), actions.as_slice());
}
