mod event_reader;
mod handlers;
mod screens;
mod tui_app;

#[cfg(test)]
mod tests {

    use std::env;

    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
    use ephact::application::dtos::responses::WorkflowListItemResponse;
    use ephact::presentation::tui::{
        ListWorkflowsHandler,
        screens::{ListWorkflowsScreen, home::HomeScreen, splash::SplashScreen},
        tui_app::{TuiApp, TuiScreen},
    };
    use ratatui::{
        Terminal,
        backend::TestBackend,
        buffer::{Buffer, Cell},
        style::{Color, Style},
    };

    use crate::common::fakes::fake_list_workflows_port::FakeListWorkflowsPort;

    struct TuiRenderAssertions;

    impl TuiRenderAssertions {
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
            let text = Self::row_text(buffer, row);
            let columns: Vec<char> = text.chars().collect();
            let wanted: Vec<char> = label.chars().collect();
            let start = columns
                .windows(wanted.len())
                .position(|window| window == wanted.as_slice())
                .expect("label on row");
            Self::row_styles(buffer, row)[start..start + wanted.len()].to_vec()
        }
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

        app.handle_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));

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
        assert_eq!(app.home_selection(), 2);
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

        app.handle_key(KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE));

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
        assert_eq!(app.list_workflows_screen().selected_index(), 0);
        app.handle_key(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
        assert_eq!(app.list_workflows_screen().selected_index(), 0);
        app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        assert_eq!(app.list_workflows_screen().selected_index(), 1);
        app.handle_key(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
        assert_eq!(app.list_workflows_screen().selected_index(), 1);
    }

    #[test]
    fn splash_renders_project_emblem() {
        let text = TuiRenderAssertions::buffer_text(&TuiRenderAssertions::rendered_buffer(
            SplashScreen::render,
        ));

        assert!(text.contains("@---o"));
    }

    #[test]
    fn home_renders_initial_menu() {
        let text =
            TuiRenderAssertions::buffer_text(&TuiRenderAssertions::rendered_buffer(|frame| {
                HomeScreen::render(frame, 0);
            }));

        assert!(text.contains("Run workflow"));
        assert!(text.contains("List workflows"));
        assert!(text.contains("List actions"));
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
        let buffer = TuiRenderAssertions::rendered_buffer(|frame| screen.render(frame));
        let text = TuiRenderAssertions::buffer_text(&buffer);

        assert!(text.contains("CI  (.github/workflows/ci.yml)  [push, pull_request]"));
        assert!(text.contains("Unnamed workflow  (-)  [-]"));
    }

    #[test]
    fn list_workflows_screen_renders_empty_message() {
        let screen = ListWorkflowsScreen::new(vec![]);
        let buffer = TuiRenderAssertions::rendered_buffer(|frame| screen.render(frame));

        assert!(
            TuiRenderAssertions::buffer_text(&buffer).contains("No workflows found in repository")
        );
    }

    #[test]
    fn list_workflows_screen_renders_highlight_on_selected_row() {
        let workflows = vec![
            WorkflowListItemResponse::new(Some("first".into()), None, vec![]),
            WorkflowListItemResponse::new(Some("second".into()), None, vec![]),
        ];
        let mut screen = ListWorkflowsScreen::new(workflows);
        screen.select_next();
        let buffer = TuiRenderAssertions::rendered_buffer(|frame| screen.render(frame));

        let selected = TuiRenderAssertions::styles_for_label(&buffer, "second");
        let unselected = TuiRenderAssertions::styles_for_label(&buffer, "first");
        assert!(selected.iter().all(|style| style.bg == Some(Color::Cyan)));
        assert!(unselected.iter().all(|style| style.bg != Some(Color::Cyan)));
    }

    #[test]
    fn home_renders_highlight_on_selected_menu_item() {
        let buffer = TuiRenderAssertions::rendered_buffer(|frame| HomeScreen::render(frame, 1));

        let selected = TuiRenderAssertions::styles_for_label(&buffer, "List workflows");
        let unselected = TuiRenderAssertions::styles_for_label(&buffer, "Run workflow");
        assert!(selected.iter().all(|style| style.bg == Some(Color::Cyan)));
        assert!(unselected.iter().all(|style| style.bg != Some(Color::Cyan)));
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
}
