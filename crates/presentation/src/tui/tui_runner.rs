use std::{path::PathBuf, sync::Arc, time::Duration};

use ratatui::{DefaultTerminal, Terminal, backend::CrosstermBackend};

use super::{
    event_reader::EventReader,
    screen_manager::TuiScreen,
    screens::{ListActionsScreen, ListWorkflowsScreen},
    terminal_guard::TerminalGuard,
    tui_app::TuiApp,
};
use crate::{
    application::{
        dtos::responses::RunSummaryResponse,
        ports::{
            inbound::{
                list_actions_port::ListActionsPort, list_workflows_port::ListWorkflowsPort,
                run_workflow_port::RunWorkflowPort,
            },
            outbound::{RunInputsDiscovererPort, SettingsStorePort},
        },
    },
    cli::TuiProgressStream,
    domain::Settings,
    handlers::{RunHandler, SingleWorkflowRun},
    infrastructure::logging::FailureLogPathStore,
};

struct RunTask {
    handle: tokio::task::JoinHandle<Result<RunSummaryResponse, String>>,
    run_id: String,
}

struct TuiLoopState {
    run_task: Option<RunTask>,
    cancelled: bool,
    input_configuration_pending: bool,
}

impl TuiLoopState {
    fn new() -> Self {
        Self {
            run_task: None,
            cancelled: false,
            input_configuration_pending: false,
        }
    }
}

pub struct TuiRunner {
    list_workflows_port: Arc<dyn ListWorkflowsPort>,
    list_actions_port: Arc<dyn ListActionsPort>,
    run_workflow_port: Arc<dyn RunWorkflowPort>,
    progress_stream: Option<TuiProgressStream>,
    discover_run_inputs_port: Option<Arc<dyn RunInputsDiscovererPort>>,
    failure_log_path_store: Option<FailureLogPathStore>,
    settings: Settings,
    settings_store: Option<Arc<dyn SettingsStorePort>>,
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
            discover_run_inputs_port: None,
            failure_log_path_store: None,
            settings: Settings::default(),
            settings_store: None,
        }
    }

    pub fn with_input_discovery(
        mut self,
        discover_run_inputs_port: Arc<dyn RunInputsDiscovererPort>,
    ) -> Self {
        self.discover_run_inputs_port = Some(discover_run_inputs_port);
        self
    }

    pub fn with_progress_stream(mut self, progress_stream: TuiProgressStream) -> Self {
        self.progress_stream = Some(progress_stream);
        self
    }
    pub fn with_failure_log_path_store(
        mut self,
        failure_log_path_store: FailureLogPathStore,
    ) -> Self {
        self.failure_log_path_store = Some(failure_log_path_store);
        self
    }
    pub fn with_settings(
        mut self,
        settings: Settings,
        store: Option<Arc<dyn SettingsStorePort>>,
    ) -> Self {
        self.settings = settings;
        self.settings_store = store;
        self
    }

    pub async fn run(&self, emblem: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut app = self.build_app(emblem)?;
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

    fn build_app(&self, emblem: &str) -> Result<TuiApp, Box<dyn std::error::Error>> {
        let current_dir = std::env::current_dir()?;
        let workflows_screen =
            ListWorkflowsScreen::from_handler(&*self.list_workflows_port, current_dir.clone())?;
        let actions_screen =
            ListActionsScreen::from_handler(&*self.list_actions_port, current_dir)?;
        Ok(
            TuiApp::new(workflows_screen.workflows().to_vec(), emblem.to_string())
                .with_actions(actions_screen.actions().to_vec())
                .with_settings(self.settings.clone(), self.settings_store.clone()),
        )
    }

    /// Runs the given workflow (or all workflows when `None`) in the current
    /// directory and returns its run summary.
    pub async fn run_workflow(
        &self,
        workflow: Option<String>,
    ) -> Result<RunSummaryResponse, Box<dyn std::error::Error>> {
        let event = None;
        RunHandler::handle_with_event_and_inputs(
            &*self.run_workflow_port,
            std::env::current_dir()?,
            workflow,
            event,
            Vec::new(),
        )
        .await
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
        let mut state = TuiLoopState::new();
        while app.screen() != TuiScreen::Exit {
            self.tick(terminal, app, &mut state).await?;
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
        state: &mut TuiLoopState,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.process_progress(app);
        Self::render_and_handle_input(terminal, app)?;
        self.process_cancel_request(app, &mut state.run_task, &mut state.cancelled);
        self.process_finished_run(app, &mut state.run_task, &mut state.cancelled)
            .await?;
        self.process_run_request(
            app,
            &mut state.run_task,
            &mut state.input_configuration_pending,
        )?;
        self.process_configured_run_request(
            app,
            &mut state.run_task,
            &mut state.input_configuration_pending,
        )?;
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
            task.handle.abort();
            self.take_failure_log_path(&task.run_id);
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
        if !task.handle.is_finished() {
            return Ok(());
        }
        let task = run_task.take().expect("finished run task");
        if *cancelled {
            self.take_failure_log_path(&task.run_id);
            *cancelled = false;
            return Ok(());
        }
        let result = task.handle.await.map_err(|error| error.to_string())?;
        match result {
            Ok(summary) => {
                let failure_log_path = self.take_failure_log_path(&task.run_id);
                app.record_run_outcome_with_failure_log_path(summary, failure_log_path);
                Ok(())
            }
            Err(error) => self.record_failed_run(app, &task.run_id, error),
        }
    }

    fn record_failed_run(
        &self,
        app: &mut TuiApp,
        run_id: &str,
        error: String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let failure_log_path = self.take_failure_log_path(run_id);
        let Some(failure_log_path) = failure_log_path else {
            return Err(error.into());
        };
        app.record_run_outcome_with_failure_log_path(
            Self::failed_summary(),
            Some(failure_log_path),
        );
        Ok(())
    }

    fn take_failure_log_path(&self, run_id: &str) -> Option<PathBuf> {
        self.failure_log_path_store
            .as_ref()
            .and_then(|store| store.take(run_id))
    }

    fn failed_summary() -> RunSummaryResponse {
        RunSummaryResponse::new("Workflow failed", Vec::new(), false, Duration::ZERO)
    }
    fn process_run_request(
        &self,
        app: &mut TuiApp,
        run_task: &mut Option<RunTask>,
        input_configuration_pending: &mut bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if run_task.is_some() || !app.take_run_request() {
            return Ok(());
        }
        *input_configuration_pending = false;
        let events = app.selected_workflow_events();
        app.begin_run_configuration(events.clone(), Vec::new());
        if events.is_empty() {
            app.report_configuration_error(
                "Error: Workflow declares no supported events".to_owned(),
            );
        }
        Ok(())
    }

    fn process_configured_run_request(
        &self,
        app: &mut TuiApp,
        run_task: &mut Option<RunTask>,
        input_configuration_pending: &mut bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let Some(configuration) = app.take_configured_run_request() else {
            return Ok(());
        };
        let workflow = app.selected_workflow_name().map(ToString::to_string);
        if self.prepare_input_configuration(
            app,
            &workflow,
            configuration.event(),
            input_configuration_pending,
        )? {
            return Ok(());
        }
        *input_configuration_pending = false;
        let port = self.run_workflow_port.clone();
        let repository_path = std::env::current_dir()?;
        app.start_run();
        *run_task = Some(Self::spawn_workflow_task(
            port,
            repository_path,
            workflow,
            configuration.event().to_string(),
            configuration.inputs().to_vec(),
        ));
        Ok(())
    }

    fn prepare_input_configuration(
        &self,
        app: &mut TuiApp,
        workflow: &Option<String>,
        event: &str,
        input_configuration_pending: &mut bool,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        if *input_configuration_pending {
            return Ok(false);
        }
        let Some(discover_port) = self.discover_run_inputs_port.as_ref() else {
            return Ok(false);
        };
        let declarations = RunHandler::discover_inputs(
            &**discover_port,
            std::env::current_dir()?,
            workflow.clone(),
            Some(event.to_string()),
        );
        match declarations {
            Ok(declarations) if !declarations.is_empty() => {
                app.begin_run_configuration(vec![event.to_string()], declarations);
                *input_configuration_pending = true;
            }
            Err(error) => {
                app.begin_run_configuration(vec![event.to_string()], Vec::new());
                app.report_configuration_error(error.to_string());
            }
            Ok(_) => return Ok(false),
        }
        Ok(true)
    }

    fn spawn_workflow_task(
        port: Arc<dyn RunWorkflowPort>,
        repository_path: PathBuf,
        workflow: Option<String>,
        event: String,
        inputs: Vec<(String, String)>,
    ) -> RunTask {
        let run_id = RunHandler::new_run_id();
        let task_run_id = run_id.clone();
        let run = SingleWorkflowRun::new(repository_path, workflow, Some(event), inputs, &run_id);
        let handle = tokio::spawn(async move {
            RunHandler::execute(&*port, run)
                .await
                .map_err(|error| error.to_string())
        });
        RunTask {
            handle,
            run_id: task_run_id,
        }
    }

    fn cancelled_summary() -> RunSummaryResponse {
        RunSummaryResponse::new("Cancelled", Vec::new(), false, Duration::ZERO)
    }
}

#[cfg(test)]
#[path = "tui_runner_tests.rs"]
mod tui_runner_tests;
