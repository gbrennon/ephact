use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ephact::presentation::tui::{
    app::{App, Screen},
    screen::{home::HomeScreen, splash::SplashScreen},
};
use ratatui::{Terminal, backend::TestBackend, buffer::Buffer};

fn rendered_buffer(render: impl FnOnce(&mut ratatui::Frame<'_>)) -> Buffer {
    let backend = TestBackend::new(80, 24);
    let mut terminal = Terminal::new(backend).expect("test terminal");
    terminal.draw(render).expect("render screen");
    terminal.backend().buffer().clone()
}

fn buffer_text(buffer: &Buffer) -> String {
    buffer
        .content()
        .iter()
        .map(|cell| cell.symbol())
        .collect::<String>()
}

#[test]
fn any_key_advances_splash_to_home() {
    let mut app = App::new();
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    assert_eq!(app.screen(), Screen::Home);
}

#[test]
fn q_exits_from_home() {
    let mut app = App::new();
    app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
    app.handle_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));
    assert_eq!(app.screen(), Screen::Exit);
}

#[test]
fn splash_renders_project_emblem() {
    let text = buffer_text(&rendered_buffer(SplashScreen::render));
    assert!(text.contains("@---o"));
}

#[test]
fn home_renders_initial_menu() {
    let text = buffer_text(&rendered_buffer(HomeScreen::render));
    assert!(text.contains("Run workflow"));
    assert!(text.contains("List workflows"));
    assert!(text.contains("List actions"));
}
