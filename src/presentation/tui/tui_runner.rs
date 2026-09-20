use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use ratatui::{DefaultTerminal, Terminal, backend::CrosstermBackend};

use crate::application::dtos::responses::RunSummaryResponse;
use crate::application::ports::inbound::list_actions_port::ListActionsPort;
use crate::application::ports::inbound::list_workflows_port::ListWorkflowsPort;
use crate::application::ports::inbound::run_workflow_port::RunWorkflowPort;
use crate::presentation::cli::TuiProgressStream;
use crate::presentation::handlers::RunHandler;

use super::event_reader::EventReader;
use super::screens::{ListActionsScreen, ListWorkflowsScreen};
use super::terminal_guard::TerminalGuard;
use super::tui_app::{TuiApp, TuiScreen};
type RunTask = tokio::task::JoinHandle<Result<RunSummaryResponse, String>>;

pub struct TuiRunner {
    list_workflows_port: Arc<dyn ListWorkflowsPort>,
    list_actions_port: Arc<dyn ListActionsPort>,
    run_workflow_port: Arc<dyn RunWorkflowPort>,
    progress_stream: Option<TuiProgressStream>,
}

impl TuiRunner {
    pub fn new(
        list_workflows_port: Arc<dyn ListWorkflowsPort>,
        list_actions_port: Arc<dyn ListActionsPort>,
        run_workflow_port: Arc<dyn RunWorkflowPort>,
    ) -> Self {
        Self {
            list_workflows_port,
            list_actions_port,
            run_workflow_port,
            progress_stream: None,
        }
    }

    pub fn with_progress_stream(mut self, progress_stream: TuiProgressStream) -> Self {
        self.progress_stream = Some(progress_stream);
        self
    }

    pub async fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut app = self.build_app()?;
        let _guard = TerminalGuard::enter()?;
        let mut terminal = Self::init_terminal()?;
        self.activate_tui_progress();
        let result = self.run_event_loop(&mut terminal, &mut app).await;
        self.deactivate_tui_progress();
        result
    }

    fn activate_tui_progress(&self) {
        if let Some(stream) = &self.progress_stream {
            stream.activate_tui();
        }
    }

    fn deactivate_tui_progress(&self) {
        if let Some(stream) = &self.progress_stream {
            stream.deactivate_tui();
        }
    }

    fn build_app(&self) -> Result<TuiApp, Box<dyn std::error::Error>> {
        let current_dir = std::env::current_dir()?;
        let workflows_screen =
            ListWorkflowsScreen::from_handler(&*self.list_workflows_port, current_dir.clone())?;
        let actions_screen =
            ListActionsScreen::from_handler(&*self.list_actions_port, current_dir)?;
        Ok(TuiApp::new(workflows_screen.workflows().to_vec())
            .with_actions(actions_screen.actions().to_vec()))
    }

    /// Runs the given workflow (or all workflows when `None`) in the current
    /// directory and returns its run summary.
    pub async fn run_workflow(
        &self,
        workflow: Option<String>,
    ) -> Result<RunSummaryResponse, Box<dyn std::error::Error>> {
        RunHandler::handle(&*self.run_workflow_port, std::env::current_dir()?, workflow).await
    }

    fn init_terminal() -> Result<DefaultTerminal, Box<dyn std::error::Error>> {
        let backend = CrosstermBackend::new(std::io::stdout());
        Ok(Terminal::new(backend)?)
    }

    async fn run_event_loop(
        &self,
        terminal: &mut DefaultTerminal,
        app: &mut TuiApp,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut run_task = None;
        let mut cancelled = false;
        while app.screen() != TuiScreen::Exit {
            self.tick(terminal, app, &mut run_task, &mut cancelled)
                .await?;
        }
        Ok(())
    }

    fn render_and_handle_input(
        terminal: &mut DefaultTerminal,
        app: &mut TuiApp,
    ) -> Result<(), Box<dyn std::error::Error>> {
        terminal.draw(|frame| app.render(frame))?;
        if let Some(key) = EventReader::read_key()? {
            app.handle_key(key);
        }
        Ok(())
    }

    async fn tick(
        &self,
        terminal: &mut DefaultTerminal,
        app: &mut TuiApp,
        run_task: &mut Option<RunTask>,
        cancelled: &mut bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.process_progress(app);
        Self::render_and_handle_input(terminal, app)?;
        self.process_cancel_request(app, run_task, cancelled);
        self.process_finished_run(app, run_task, cancelled).await?;
        self.process_run_request(app, run_task)?;
        Ok(())
    }

    fn process_progress(&self, app: &mut TuiApp) {
        let Some(stream) = &self.progress_stream else {
            return;
        };
        while let Some(line) = stream.try_recv() {
            app.record_progress(line);
        }
    }
    fn process_cancel_request(
        &self,
        app: &mut TuiApp,
        run_task: &mut Option<RunTask>,
        cancelled: &mut bool,
    ) {
        if !app.take_cancel_request() {
            return;
        }
        *cancelled = true;
        if let Some(task) = run_task.as_ref() {
            task.abort();
        }
        app.record_run_outcome(Self::cancelled_summary());
    }

    async fn process_finished_run(
        &self,
        app: &mut TuiApp,
        run_task: &mut Option<RunTask>,
        cancelled: &mut bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let Some(task) = run_task.as_ref() else {
            return Ok(());
        };
        if !task.is_finished() {
            return Ok(());
        }
        let task = run_task.take().expect("finished run task");
        let result = task.await;
        if *cancelled {
            *cancelled = false;
            return Ok(());
        }
        let summary = result
            .map_err(|error| error.to_string())?
            .map_err(|error| error.to_string())?;
        app.record_run_outcome(summary);
        Ok(())
    }

    fn process_run_request(
        &self,
        app: &mut TuiApp,
        run_task: &mut Option<RunTask>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if run_task.is_some() || !app.take_run_request() {
            return Ok(());
        }
        let workflow = app
            .run_workflow_screen()
            .selected_workflow_name()
            .map(ToString::to_string);
        let port = self.run_workflow_port.clone();
        let repository_path = std::env::current_dir()?;
        app.start_run();
        *run_task = Some(Self::spawn_workflow_task(port, repository_path, workflow));
        Ok(())
    }

    fn spawn_workflow_task(
        port: Arc<dyn RunWorkflowPort>,
        repository_path: PathBuf,
        workflow: Option<String>,
    ) -> RunTask {
        tokio::spawn(async move {
            RunHandler::handle(&*port, repository_path, workflow)
                .await
                .map_err(|error| error.to_string())
        })
    }

    fn cancelled_summary() -> RunSummaryResponse {
        RunSummaryResponse::new("Cancelled", Vec::new(), false, Duration::ZERO)
    }
}
