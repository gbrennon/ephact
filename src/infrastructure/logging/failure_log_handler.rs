use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use crate::{
    domain::{
        messages::events::{DomainEvent, StepFinishedPayload},
        value_objects::RepositoryName,
    },
    infrastructure::messaging::DomainEventHandler,
};

/// Shared status for filesystem failures encountered while writing diagnostics.
#[derive(Clone, Default)]
pub struct FailureLogErrorStore {
    errors: Arc<Mutex<Vec<String>>>,
}

impl FailureLogErrorStore {
    /// Creates an empty error store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a log-write failure for reporting by the CLI.
    pub fn record(&self, error: impl Into<String>) {
        if let Ok(mut errors) = self.errors.lock() {
            errors.push(error.into());
        }
    }

    /// Returns and clears all errors recorded since the previous read.
    pub fn read_and_clear(&self) -> Vec<String> {
        self.errors
            .lock()
            .map(|mut errors| std::mem::take(&mut *errors))
            .unwrap_or_default()
    }
}

/// Shared store of diagnostics paths created during a run.
#[derive(Clone, Default)]
pub struct FailureLogPathStore {
    paths: Arc<Mutex<HashMap<String, PathBuf>>>,
}

impl FailureLogPathStore {
    /// Creates an empty path store.
    pub fn new() -> Self {
        Self::default()
    }

    fn record(&self, run_id: &str, path: PathBuf) {
        if let Ok(mut paths) = self.paths.lock() {
            paths.insert(run_id.to_string(), path);
        }
    }

    /// Returns and removes the diagnostics path for a run, if one was written.
    pub fn take(&self, run_id: &str) -> Option<PathBuf> {
        self.paths.lock().ok()?.remove(run_id)
    }

    /// Returns and clears every path currently held by the store.
    pub fn read_and_clear(&self) -> HashMap<String, PathBuf> {
        self.paths
            .lock()
            .map(|mut paths| std::mem::take(&mut *paths))
            .unwrap_or_default()
    }
}

/// Shared stores used to report failure diagnostics.
#[derive(Clone)]
pub struct FailureLogStores {
    error_store: FailureLogErrorStore,
    path_store: FailureLogPathStore,
}

impl FailureLogStores {
    /// Creates empty stores for failure diagnostics.
    pub fn new() -> Self {
        Self {
            error_store: FailureLogErrorStore::new(),
            path_store: FailureLogPathStore::new(),
        }
    }

    /// Returns the shared log-write error store.
    pub fn error_store(&self) -> FailureLogErrorStore {
        self.error_store.clone()
    }

    /// Combines caller-owned stores for composition-root wiring.
    pub fn from_stores(error_store: FailureLogErrorStore, path_store: FailureLogPathStore) -> Self {
        Self {
            error_store,
            path_store,
        }
    }

    /// Returns the shared diagnostics path store.
    pub fn path_store(&self) -> FailureLogPathStore {
        self.path_store.clone()
    }
}

impl Default for FailureLogStores {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
struct FailedStepRecord {
    workflow_name: String,
    job_id: String,
    step_name: String,
    exit_code: Option<i64>,
    stdout: String,
    stderr: String,
}

#[derive(Debug, Clone)]
struct EarlyFailureRecord {
    workflow_name: Option<String>,
    error: String,
}

#[derive(Debug, Clone)]
struct FailureLogState {
    repository_path: String,
    failed_steps: Vec<FailedStepRecord>,
    early_errors: Vec<EarlyFailureRecord>,
}
impl FailureLogState {
    fn new(repository_path: String) -> Self {
        Self {
            repository_path,
            failed_steps: Vec::new(),
            early_errors: Vec::new(),
        }
    }
}

/// Persists failed workflow details outside the repository being executed.
///
/// The handler buffers step failures until the run completes, but flushes an
/// early run failure immediately so that errors occurring before a summary is
/// available are not lost.
pub struct FailureLogHandler {
    states: Arc<Mutex<HashMap<String, FailureLogState>>>,
    temp_root: PathBuf,
    errors: FailureLogErrorStore,
    paths: FailureLogPathStore,
}

impl FailureLogHandler {
    /// Creates a handler writing below the process system temporary directory.
    pub fn new() -> Self {
        Self::with_temp_root(std::env::temp_dir())
    }

    /// Creates a handler writing below `temp_root`.
    pub fn with_temp_root(temp_root: impl Into<PathBuf>) -> Self {
        Self::with_temp_root_and_stores(
            temp_root,
            FailureLogErrorStore::new(),
            FailureLogPathStore::new(),
        )
    }

    /// Creates a handler with caller-owned status stores for composition-root wiring.
    pub fn with_temp_root_and_stores(
        temp_root: impl Into<PathBuf>,
        errors: FailureLogErrorStore,
        paths: FailureLogPathStore,
    ) -> Self {
        Self {
            states: Arc::new(Mutex::new(HashMap::new())),
            temp_root: temp_root.into(),
            errors,
            paths,
        }
    }

    /// Returns the shared error status store used by this handler.
    pub fn error_store(&self) -> FailureLogErrorStore {
        self.errors.clone()
    }

    /// Returns the shared path status store used by this handler.
    pub fn path_store(&self) -> FailureLogPathStore {
        self.paths.clone()
    }

    fn on_run_started(&self, run_id: &str, repository_path: &str) {
        if let Ok(mut states) = self.states.lock() {
            states.insert(
                run_id.to_string(),
                FailureLogState::new(repository_path.to_string()),
            );
        }
    }

    fn on_step_finished(&self, payload: &StepFinishedPayload) {
        if payload.success() {
            return;
        }
        let Ok(mut states) = self.states.lock() else {
            return;
        };
        let Some(state) = states.get_mut(payload.run_id()) else {
            return;
        };
        state.failed_steps.push(FailedStepRecord {
            workflow_name: payload.workflow_name().to_string(),
            job_id: payload.job_id().to_string(),
            step_name: payload.step_name().to_string(),
            exit_code: payload.exit_code(),
            stdout: payload.stdout().to_string(),
            stderr: payload.stderr().to_string(),
        });
    }

    fn on_run_failed(
        &self,
        run_id: &str,
        repository_path: &str,
        workflow_name: Option<&str>,
        error: &str,
    ) {
        let state = if let Ok(mut states) = self.states.lock() {
            let mut state = states
                .remove(run_id)
                .unwrap_or_else(|| FailureLogState::new(repository_path.to_string()));
            if state.repository_path.is_empty() {
                state.repository_path = repository_path.to_string();
            }
            state.early_errors.push(EarlyFailureRecord {
                workflow_name: workflow_name.map(str::to_string),
                error: error.to_string(),
            });
            state
        } else {
            return;
        };
        self.flush(run_id, state);
    }

    fn on_run_completed(&self, run_id: &str, repository_path: &str, success: bool) {
        let state = self
            .states
            .lock()
            .ok()
            .and_then(|mut states| states.remove(run_id));
        if success {
            return;
        }
        let state = state.unwrap_or_else(|| FailureLogState::new(repository_path.to_string()));
        self.flush(run_id, state);
    }

    fn flush(&self, run_id: &str, state: FailureLogState) {
        match self.write_log(run_id, &state) {
            Ok(path) => self.paths.record(run_id, path),
            Err(error) => self.errors.record(error),
        }
    }

    fn write_log(&self, run_id: &str, state: &FailureLogState) -> Result<PathBuf, String> {
        let repository_name = repository_name(&state.repository_path)?;
        let run_component = safe_filename(run_id);
        let directory = self.temp_root.join("ephact").join(repository_name.as_str());
        let path = directory.join(format!("{run_component}.log"));
        if directory.starts_with(Path::new(&state.repository_path)) {
            return Err(format!(
                "refusing to write failure log '{}' under repository '{}'",
                path.display(),
                state.repository_path
            ));
        }
        fs::create_dir_all(&directory).map_err(|error| {
            format!(
                "failed to create failure log directory '{}': {error}",
                directory.display()
            )
        })?;
        fs::write(&path, render_log(run_id, state)).map_err(|error| {
            format!("failed to write failure log '{}': {error}", path.display())
        })?;
        Ok(path)
    }
}

impl Default for FailureLogHandler {
    fn default() -> Self {
        Self::new()
    }
}

impl DomainEventHandler for FailureLogHandler {
    fn handle(&self, event: &DomainEvent) {
        match event {
            DomainEvent::RunStarted(payload) => {
                self.on_run_started(payload.run_id(), payload.repository_path())
            }
            DomainEvent::StepFinished(payload) => self.on_step_finished(payload),
            DomainEvent::RunFailed(payload) => self.on_run_failed(
                payload.run_id(),
                payload.repository_path(),
                payload.workflow_name(),
                payload.error(),
            ),
            DomainEvent::ActRunCompleted(payload) => self.on_run_completed(
                payload.run_id(),
                payload.repository_path(),
                payload.success(),
            ),
            _ => {}
        }
    }
}

fn repository_name(repository_path: &str) -> Result<RepositoryName, String> {
    let path = Path::new(repository_path);
    let name = path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default()
        .to_string();
    RepositoryName::new(name).map_err(|error| {
        format!(
            "failed to derive failure log repository name from '{}': {error:?}",
            path.display()
        )
    })
}

fn safe_filename(run_id: &str) -> String {
    let sanitized: String = run_id
        .chars()
        .map(|character| match character {
            '/' | '\\' => '_',
            _ => character,
        })
        .collect();
    if sanitized.is_empty() {
        "unknown-run".to_string()
    } else {
        sanitized
    }
}

fn render_log(run_id: &str, state: &FailureLogState) -> String {
    let mut output = format!("Repository: {}\nRun ID: {run_id}\n", state.repository_path);
    for step in &state.failed_steps {
        output.push_str("\nFailed step\n");
        output.push_str(&format!("Workflow: {}\n", step.workflow_name));
        output.push_str(&format!("Job: {}\n", step.job_id));
        output.push_str(&format!("Step: {}\n", step.step_name));
        output.push_str(&format!(
            "Exit code: {}\n",
            step.exit_code
                .map_or_else(|| "none".to_string(), |code| code.to_string())
        ));
        output.push_str("stdout:\n");
        output.push_str(&step.stdout);
        if !step.stdout.ends_with('\n') {
            output.push('\n');
        }
        output.push_str("stderr:\n");
        output.push_str(&step.stderr);
        if !step.stderr.ends_with('\n') {
            output.push('\n');
        }
    }
    for failure in &state.early_errors {
        output.push_str("\nEarly error\n");
        if let Some(workflow_name) = &failure.workflow_name {
            output.push_str(&format!("Workflow: {workflow_name}\n"));
        }
        output.push_str(&format!("Error: {}\n", failure.error));
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::messages::events::{
        ActRunCompletedPayload, RunFailedPayload, RunStartedPayload, StepFinishedDetails,
    };

    fn started(run_id: &str, repository_path: &str) -> DomainEvent {
        DomainEvent::RunStarted(RunStartedPayload::new(
            run_id.to_string(),
            repository_path.to_string(),
        ))
    }

    fn finished(run_id: &str) -> DomainEvent {
        DomainEvent::StepFinished(StepFinishedPayload::new(
            run_id.to_string(),
            StepFinishedDetails::new(
                "Build".to_string(),
                "build".to_string(),
                "compile".to_string(),
                false,
                Some(1),
            )
            .with_stdout("stdout details".to_string())
            .with_stderr("stderr details".to_string()),
        ))
    }

    #[test]
    fn failed_step_flushes_details_outside_repository() {
        let temp_root = tempfile::tempdir().unwrap();
        let handler = FailureLogHandler::with_temp_root(temp_root.path());
        handler.handle(&started("run-1", "/repo/project"));
        handler.handle(&finished("run-1"));
        handler.handle(&DomainEvent::ActRunCompleted(ActRunCompletedPayload::new(
            "run-1".to_string(),
            "/repo/project".to_string(),
            Vec::new(),
            false,
        )));

        let path = handler.path_store().take("run-1").unwrap();
        assert!(!path.starts_with("/repo/project"));
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("Workflow: Build"));
        assert!(content.contains("stdout details"));
        assert!(content.contains("stderr details"));
    }

    #[test]
    fn early_failure_flushes_immediately() {
        let temp_root = tempfile::tempdir().unwrap();
        let handler = FailureLogHandler::with_temp_root(temp_root.path());
        handler.handle(&started("run-2", "/repo/project"));
        handler.handle(&DomainEvent::RunFailed(RunFailedPayload::new(
            "run-2".to_string(),
            "/repo/project".to_string(),
            None,
            "workflow could not be read".to_string(),
        )));

        let path = handler.path_store().take("run-2").unwrap();
        assert!(
            fs::read_to_string(path)
                .unwrap()
                .contains("workflow could not be read")
        );
    }

    #[test]
    fn successful_completion_discards_buffered_state() {
        let temp_root = tempfile::tempdir().unwrap();
        let handler = FailureLogHandler::with_temp_root(temp_root.path());
        handler.handle(&started("run-3", "/repo/project"));
        handler.handle(&finished("run-3"));
        handler.handle(&DomainEvent::ActRunCompleted(ActRunCompletedPayload::new(
            "run-3".to_string(),
            "/repo/project".to_string(),
            Vec::new(),
            true,
        )));

        assert!(handler.path_store().take("run-3").is_none());
        assert!(handler.errors.read_and_clear().is_empty());
    }
}
