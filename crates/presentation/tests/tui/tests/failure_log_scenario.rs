use std::{
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use super::{
    empty_list_actions_port::EmptyListActionsPort,
    empty_list_workflows_port::EmptyListWorkflowsPort,
    unused_run_workflow_port::UnusedRunWorkflowPort,
};
use crate::{
    application::ports::outbound::DomainEventHandlerPort,
    domain::messages::events::{Event, RunFailedPayload, RunStartedPayload},
    infrastructure::logging::FailureLogHandler,
    tui::{TuiApp, TuiRunner},
};

static NEXT_SCENARIO_ID: AtomicU64 = AtomicU64::new(0);

pub struct FailureLogScenario {
    temp_root: PathBuf,
    pub(crate) handler: FailureLogHandler,
    pub runner: TuiRunner,
    pub app: TuiApp,
}

impl FailureLogScenario {
    pub fn new(run_ids: &[&str]) -> Self {
        let sequence = NEXT_SCENARIO_ID.fetch_add(1, Ordering::Relaxed);
        let temp_root = std::env::temp_dir().join(format!(
            "ephact-tui-failure-{}-{sequence}",
            std::process::id()
        ));
        let handler = FailureLogHandler::with_temp_root(&temp_root);
        for run_id in run_ids {
            handler.handle(&Event::RunStarted(RunStartedPayload::new(
                (*run_id).to_string(),
                "/repo/project".to_string(),
            )));
            handler.handle(&Event::RunFailed(RunFailedPayload::new(
                (*run_id).to_string(),
                "/repo/project".to_string(),
                None,
                "workflow could not be read".to_string(),
            )));
        }
        let runner = TuiRunner::new(
            Arc::new(EmptyListWorkflowsPort),
            Arc::new(EmptyListActionsPort),
            Arc::new(UnusedRunWorkflowPort),
        )
        .with_failure_log_path_store(handler.path_store());
        let mut app = TuiApp::new(Vec::new(), String::new());
        app.handle_key(KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE));
        app.handle_key(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
        app.start_run();
        Self {
            temp_root,
            handler,
            runner,
            app,
        }
    }

    pub fn path(&self, run_id: &str) -> PathBuf {
        self.temp_root
            .join("ephact")
            .join("project")
            .join(format!("failure-{run_id}.log"))
    }

    pub fn cleanup(self) {
        std::fs::remove_dir_all(self.temp_root).expect("test diagnostics should be removed");
    }
}
