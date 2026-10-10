use std::{
    collections::HashMap,
    fs,
    io::Read,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use crate::{
    application::ports::outbound::DomainEventHandlerPort,
    domain::{
        messages::events::{Event, StepFinishedPayload},
        value_objects::RepositoryName,
    },
};

const DEFAULT_FAILURE_LOG_RETENTION_HOURS: u64 = 24;
const FAILURE_LOG_MARKER: &str = "ephact-failure-log-v1\n";

/// Shared runtime configuration for failure-log retention.
#[derive(Clone)]
pub struct FailureLogRetentionStore {
    hours: Arc<Mutex<u64>>,
}

impl Default for FailureLogRetentionStore {
    fn default() -> Self {
        Self {
            hours: Arc::new(Mutex::new(DEFAULT_FAILURE_LOG_RETENTION_HOURS)),
        }
    }
}

impl FailureLogRetentionStore {
    /// Creates a retention store with the default number of hours.
    pub fn new() -> Self {
        Self::default()
    }

    /// Applies the active failure-log retention in hours.
    pub fn apply_hours(&self, hours: u64) {
        if let Ok(mut current) = self.hours.lock() {
            *current = hours;
        }
    }

    /// Returns the active failure-log retention in hours.
    pub fn hours(&self) -> u64 {
        self.hours
            .lock()
            .map(|hours| *hours)
            .unwrap_or(DEFAULT_FAILURE_LOG_RETENTION_HOURS)
    }

    fn retention_duration(hours: u64) -> Duration {
        Duration::from_secs(hours.saturating_mul(60 * 60))
    }
}

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
    retention_store: FailureLogRetentionStore,
}

impl FailureLogStores {
    /// Creates empty stores for failure diagnostics.
    pub fn new() -> Self {
        Self {
            error_store: FailureLogErrorStore::new(),
            path_store: FailureLogPathStore::new(),
            retention_store: FailureLogRetentionStore::new(),
        }
    }

    /// Returns the shared log-write error store.
    pub fn error_store(&self) -> FailureLogErrorStore {
        self.error_store.clone()
    }

    /// Combines caller-owned stores for composition-root wiring.
    pub fn from_stores(
        error_store: FailureLogErrorStore,
        path_store: FailureLogPathStore,
        retention_store: FailureLogRetentionStore,
    ) -> Self {
        Self {
            error_store,
            path_store,
            retention_store,
        }
    }

    /// Returns the shared diagnostics path store.
    pub fn path_store(&self) -> FailureLogPathStore {
        self.path_store.clone()
    }

    /// Returns the shared retention configuration store.
    pub fn retention_store(&self) -> FailureLogRetentionStore {
        self.retention_store.clone()
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
    retention: FailureLogRetentionStore,
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
            FailureLogRetentionStore::new(),
        )
    }

    /// Creates a handler with caller-owned status stores for composition-root wiring.
    pub fn with_temp_root_and_stores(
        temp_root: impl Into<PathBuf>,
        errors: FailureLogErrorStore,
        paths: FailureLogPathStore,
        retention: FailureLogRetentionStore,
    ) -> Self {
        Self {
            states: Arc::new(Mutex::new(HashMap::new())),
            temp_root: temp_root.into(),
            errors,
            paths,
            retention,
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

    fn prune_expired_logs(&self, now: SystemTime, retention: Duration) {
        let root = self.temp_root.join("ephact");
        let Ok(repositories) = fs::read_dir(root) else {
            return;
        };
        for repository in repositories.flatten() {
            self.prune_repository_logs(repository, now, retention);
        }
    }

    fn prune_repository_logs(
        &self,
        repository: fs::DirEntry,
        now: SystemTime,
        retention: Duration,
    ) {
        let Ok(repository_type) = repository.file_type() else {
            return;
        };
        if !repository_type.is_dir() {
            return;
        }
        let Ok(logs) = fs::read_dir(repository.path()) else {
            return;
        };
        for log in logs.flatten() {
            self.prune_log(log, now, retention);
        }
    }

    fn prune_log(&self, log: fs::DirEntry, now: SystemTime, retention: Duration) {
        if !Self::is_expired_log(&log, now, retention) {
            return;
        }
        self.remove_expired_log(&log.path());
    }

    fn remove_expired_log(&self, path: &Path) {
        if let Err(error) = fs::remove_file(path)
            && error.kind() != std::io::ErrorKind::NotFound
        {
            self.errors.record(format!(
                "failed to remove expired failure log '{}': {error}",
                path.display()
            ));
        }
    }

    fn is_expired_log(log: &fs::DirEntry, now: SystemTime, retention: Duration) -> bool {
        let Ok(file_type) = log.file_type() else {
            return false;
        };
        if !file_type.is_file() {
            return false;
        }
        let path = log.path();
        let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
            return false;
        };
        if !file_name.starts_with("failure-") || !file_name.ends_with(".log") {
            return false;
        }
        if !Self::has_ownership_marker(&path) {
            return false;
        }
        let Ok(modified) = log.metadata().and_then(|metadata| metadata.modified()) else {
            return false;
        };
        now.duration_since(modified)
            .map(|age| age > retention)
            .unwrap_or(false)
    }

    fn has_ownership_marker(path: &Path) -> bool {
        let Ok(mut file) = fs::File::open(path) else {
            return false;
        };
        let mut marker = [0; FAILURE_LOG_MARKER.len()];
        file.read_exact(&mut marker).is_ok() && marker.as_slice() == FAILURE_LOG_MARKER.as_bytes()
    }

    fn on_run_started(&self, run_id: &str, repository_path: &str) {
        self.prune_expired_logs(
            SystemTime::now(),
            FailureLogRetentionStore::retention_duration(self.retention.hours()),
        );
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
        let path = directory.join(format!("failure-{run_component}.log"));
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

impl DomainEventHandlerPort for FailureLogHandler {
    fn handle(&self, event: &Event) {
        match event {
            Event::RunStarted(payload) => {
                self.on_run_started(payload.run_id(), payload.repository_path())
            }
            Event::StepFinished(payload) => self.on_step_finished(payload),
            Event::RunFailed(payload) => self.on_run_failed(
                payload.run_id(),
                payload.repository_path(),
                payload.workflow_name(),
                payload.error(),
            ),
            Event::WorkflowRunCompleted(payload) => self.on_run_completed(
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
    let mut output = format!(
        "{FAILURE_LOG_MARKER}Repository: {}\nRun ID: {run_id}\n",
        state.repository_path
    );
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
        RunFailedPayload, RunStartedPayload, StepFinishedDetails, WorkflowRunCompletedPayload,
    };

    fn started(run_id: &str, repository_path: &str) -> Event {
        Event::RunStarted(RunStartedPayload::new(
            run_id.to_string(),
            repository_path.to_string(),
        ))
    }

    fn finished(run_id: &str) -> Event {
        Event::StepFinished(StepFinishedPayload::new(
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
        handler.handle(&Event::WorkflowRunCompleted(
            WorkflowRunCompletedPayload::new(
                "run-1".to_string(),
                "/repo/project".to_string(),
                Vec::new(),
                false,
            ),
        ));

        let path = handler.path_store().take("run-1").unwrap();
        assert!(!path.starts_with("/repo/project"));
        let content = fs::read_to_string(&path).unwrap();
        assert!(content.contains("Workflow: Build"));
        assert!(content.contains("stdout details"));
        assert!(content.contains("stderr details"));
        assert_eq!(
            path.file_name().and_then(|name| name.to_str()),
            Some("failure-run-1.log")
        );
    }

    #[test]
    fn generated_expired_log_is_pruned() {
        let temp_root = tempfile::tempdir().unwrap();
        let handler = FailureLogHandler::with_temp_root(temp_root.path());
        handler.handle(&started("run-generated", "/repo/project"));
        handler.handle(&finished("run-generated"));
        handler.handle(&Event::WorkflowRunCompleted(
            WorkflowRunCompletedPayload::new(
                "run-generated".to_string(),
                "/repo/project".to_string(),
                Vec::new(),
                false,
            ),
        ));

        let generated_log = handler.path_store().take("run-generated").unwrap();
        assert!(
            fs::read_to_string(&generated_log)
                .unwrap()
                .starts_with(FAILURE_LOG_MARKER)
        );
        let old = SystemTime::now() - Duration::from_secs(25 * 60 * 60);
        fs::File::open(&generated_log)
            .unwrap()
            .set_modified(old)
            .unwrap();

        handler.handle(&started("trigger-run", "/repo/project"));

        assert!(!generated_log.exists());
    }

    #[test]
    fn expired_unmarked_failure_log_is_not_pruned() {
        let temp_root = tempfile::tempdir().unwrap();
        let log_directory = temp_root.path().join("ephact/project");
        fs::create_dir_all(&log_directory).unwrap();
        let arbitrary_log = log_directory.join("failure-user-created.log");
        fs::write(&arbitrary_log, "user-created").unwrap();
        let old = SystemTime::now() - Duration::from_secs(25 * 60 * 60);
        fs::File::open(&arbitrary_log)
            .unwrap()
            .set_modified(old)
            .unwrap();

        let handler = FailureLogHandler::with_temp_root(temp_root.path());
        handler.handle(&started("trigger-run", "/repo/project"));

        assert!(arbitrary_log.exists());
    }

    #[test]
    fn unrelated_expired_log_is_not_pruned() {
        let temp_root = tempfile::tempdir().unwrap();
        let log_directory = temp_root.path().join("ephact/project");
        fs::create_dir_all(&log_directory).unwrap();
        let unrelated_log = log_directory.join("expired.log");
        let owned_log = log_directory.join("failure-run.log");
        fs::write(&unrelated_log, "unrelated").unwrap();
        fs::write(&owned_log, FAILURE_LOG_MARKER).unwrap();
        let old = SystemTime::now() - Duration::from_secs(25 * 60 * 60);
        fs::File::open(&unrelated_log)
            .unwrap()
            .set_modified(old)
            .unwrap();
        fs::File::open(&owned_log)
            .unwrap()
            .set_modified(old)
            .unwrap();

        let handler = FailureLogHandler::with_temp_root(temp_root.path());
        handler.handle(&started("trigger-run", "/repo/project"));

        assert!(unrelated_log.exists());
        assert!(!owned_log.exists());
    }

    #[test]
    fn configured_retention_controls_pruning_boundary() {
        let temp_root = tempfile::tempdir().unwrap();
        let log_directory = temp_root.path().join("ephact/project");
        fs::create_dir_all(&log_directory).unwrap();
        let owned_log = log_directory.join("failure-run.log");
        fs::write(&owned_log, FAILURE_LOG_MARKER).unwrap();
        let old = SystemTime::now() - Duration::from_secs(2 * 60 * 60);
        fs::File::open(&owned_log)
            .unwrap()
            .set_modified(old)
            .unwrap();

        let retention = FailureLogRetentionStore::new();
        retention.apply_hours(1);
        let handler = FailureLogHandler::with_temp_root_and_stores(
            temp_root.path(),
            FailureLogErrorStore::new(),
            FailureLogPathStore::new(),
            retention,
        );
        handler.handle(&started("trigger-run", "/repo/project"));

        assert!(!owned_log.exists());
    }

    #[test]
    fn missing_expired_owned_log_is_idempotent() {
        let temp_root = tempfile::tempdir().unwrap();
        let log_directory = temp_root.path().join("ephact/project");
        fs::create_dir_all(&log_directory).unwrap();
        let owned_log = log_directory.join("failure-race.log");
        fs::write(&owned_log, "owned").unwrap();
        let old = SystemTime::now() - Duration::from_secs(25 * 60 * 60);
        fs::File::open(&owned_log)
            .unwrap()
            .set_modified(old)
            .unwrap();

        let handler = FailureLogHandler::with_temp_root(temp_root.path());
        fs::remove_file(&owned_log).unwrap();
        handler.remove_expired_log(&owned_log);

        assert!(handler.error_store().read_and_clear().is_empty());
    }

    #[test]
    fn retention_duration_converts_hours_without_overflow() {
        assert_eq!(
            FailureLogRetentionStore::retention_duration(2),
            Duration::from_secs(2 * 60 * 60)
        );
        assert_eq!(
            FailureLogRetentionStore::retention_duration(u64::MAX),
            Duration::from_secs(u64::MAX)
        );
    }

    #[test]
    fn retention_store_defaults_to_24_hours_and_shares_updates() {
        let store = FailureLogRetentionStore::new();
        assert_eq!(store.hours(), 24);

        let clone = store.clone();
        clone.apply_hours(72);

        assert_eq!(store.hours(), 72);
    }

    #[test]
    fn early_failure_flushes_immediately() {
        let temp_root = tempfile::tempdir().unwrap();
        let handler = FailureLogHandler::with_temp_root(temp_root.path());
        handler.handle(&started("run-2", "/repo/project"));
        handler.handle(&Event::RunFailed(RunFailedPayload::new(
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
        handler.handle(&Event::WorkflowRunCompleted(
            WorkflowRunCompletedPayload::new(
                "run-3".to_string(),
                "/repo/project".to_string(),
                Vec::new(),
                true,
            ),
        ));

        assert!(handler.path_store().take("run-3").is_none());
        assert!(handler.errors.read_and_clear().is_empty());
    }
}
