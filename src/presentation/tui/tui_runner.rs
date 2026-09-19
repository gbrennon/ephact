use std::sync::Arc;

use ratatui::{DefaultTerminal, Terminal, backend::CrosstermBackend};

use crate::application::ports::inbound::list_workflows_port::ListWorkflowsPort;

use super::event_reader::EventReader;
use super::handlers::list_workflows_handler::ListWorkflowsHandler;
use super::terminal_guard::TerminalGuard;
use super::tui_app::{TuiApp, TuiScreen};

#[derive(Clone)]
pub struct TuiRunner {
    list_workflows_port: Arc<dyn ListWorkflowsPort>,
}

impl TuiRunner {
    pub fn new(list_workflows_port: Arc<dyn ListWorkflowsPort>) -> Self {
        Self {
            list_workflows_port,
        }
    }

    fn init_terminal() -> Result<DefaultTerminal, Box<dyn std::error::Error>> {
        let backend = CrosstermBackend::new(std::io::stdout());
        Ok(Terminal::new(backend)?)
    }

    fn run_event_loop(
        terminal: &mut DefaultTerminal,
        app: &mut TuiApp,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while app.screen() != TuiScreen::Exit {
            terminal.draw(|frame| app.render(frame))?;
            app.handle_key(EventReader::read_key()?);
        }

        Ok(())
    }

    pub fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let current_dir = std::env::current_dir()?;
        let response = ListWorkflowsHandler::handle(&*self.list_workflows_port, current_dir)?;
        let _guard = TerminalGuard::enter()?;
        let mut terminal = Self::init_terminal()?;
        let mut app = TuiApp::new(response.into_workflows());
        Self::run_event_loop(&mut terminal, &mut app)
    }
}
