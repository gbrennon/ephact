use std::{
    collections::HashMap,
    path::Path,
    process,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::SystemTime,
};

use ephact_application::{
    dtos::{
        requests::{PrepareJobContainerRequest, RunActionRequest, RunActionRequestInput},
        responses::ExecuteActionResponse,
    },
    errors::RunActionError,
    ports::{
        inbound::RunActionPort,
        outbound::{DomainEventPublisherPort, JobContainerPreparerPort},
    },
};
use ephact_domain::{
    errors::StepError,
    messages::events::{
        ContainerStartedPayload, Event, RunFailedPayload, RunStartedPayload, StepFinishedDetails,
        StepFinishedPayload, WorkflowRunCompletedPayload,
    },
};

use super::RunActionFactory;
use crate::{logging::FailureLogPathStore, steps::JsonStepTextCodec};

/// Runs a standalone action in a prepared job container and emits the same
/// lifecycle events used by workflow execution for cleanup and diagnostics.
pub struct StandaloneRunActionAdapter {
    preparer: Box<dyn JobContainerPreparerPort>,
    action_factory: RunActionFactory,
    events: Arc<dyn DomainEventPublisherPort>,
    failure_log_path_store: Option<FailureLogPathStore>,
}

static NEXT_ACTION_RUN_ID: AtomicU64 = AtomicU64::new(0);

impl StandaloneRunActionAdapter {
    pub fn new(
        preparer: Box<dyn JobContainerPreparerPort>,
        action_factory: RunActionFactory,
        events: Arc<dyn DomainEventPublisherPort>,
    ) -> Self {
        Self {
            preparer,
            action_factory,
            events,
            failure_log_path_store: None,
        }
    }

    pub fn with_failure_log_path_store(mut self, store: FailureLogPathStore) -> Self {
        self.failure_log_path_store = Some(store);
        self
    }

    fn dependency_action_ref(repository_path: &Path, action_ref: &str) -> Option<String> {
        if !action_ref.starts_with("./") {
            return None;
        }
        [
            ".forgejo/actions/install-dependencies",
            ".github/actions/install-dependencies",
        ]
        .into_iter()
        .map(Path::new)
        .find(|relative_path| {
            repository_path
                .join(relative_path)
                .join("action.yml")
                .is_file()
                && action_ref != format!("./{}", relative_path.display())
        })
        .map(|relative_path| format!("./{}", relative_path.display()))
    }

    fn run_id() -> String {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_or(0, |duration| duration.as_nanos());
        let sequence = NEXT_ACTION_RUN_ID.fetch_add(1, Ordering::Relaxed);
        format!("run-{}-{timestamp}-{sequence}", process::id())
    }
}

impl RunActionPort for StandaloneRunActionAdapter {
    fn execute(&self, request: RunActionRequest) -> Result<ExecuteActionResponse, RunActionError> {
        let run_id = Self::run_id();
        let repository_path = request.repo_path().to_path_buf();
        let action_ref = request.action_ref().to_owned();
        self.events
            .publish(Event::RunStarted(RunStartedPayload::new(
                run_id.clone(),
                repository_path.display().to_string(),
            )));
        let prepared = match self.preparer.prepare(PrepareJobContainerRequest::new(
            run_id.clone(),
            None,
            repository_path.clone(),
            false,
        )) {
            Ok(prepared) => prepared,
            Err(error) => {
                let error = RunActionError::Step(StepError::new(error.to_string()));
                self.events.publish(Event::RunFailed(RunFailedPayload::new(
                    run_id,
                    repository_path.display().to_string(),
                    None,
                    error.to_string(),
                )));
                return Err(error);
            }
        };
        let container_name = prepared.container_name().to_owned();
        let container = prepared.container_handle();
        self.events
            .publish(Event::ContainerStarted(ContainerStartedPayload::new(
                run_id.clone(),
                container_name.clone(),
            )));

        if let Some(dependency_ref) = Self::dependency_action_ref(&repository_path, &action_ref) {
            let dependency_input = RunActionRequestInput::for_action(
                dependency_ref.clone(),
                HashMap::new(),
                repository_path.clone(),
                &JsonStepTextCodec,
            )
            .map_err(RunActionError::Step)?;
            let dependency_result = (self.action_factory)(container.clone())
                .execute(RunActionRequest::new(dependency_input));
            match dependency_result {
                Ok(response) if response.exit_code() == 0 => {}
                Ok(response) => {
                    self.events
                        .publish(Event::StepFinished(StepFinishedPayload::new(
                            run_id.clone(),
                            StepFinishedDetails::new(
                                "standalone action".to_owned(),
                                "standalone".to_owned(),
                                dependency_ref,
                                false,
                                Some(response.exit_code()),
                            )
                            .with_stdout(response.stdout().to_owned())
                            .with_stderr(response.stderr().to_owned()),
                        )));
                    self.events.publish(Event::WorkflowRunCompleted(
                        WorkflowRunCompletedPayload::new(
                            run_id.clone(),
                            repository_path.display().to_string(),
                            vec![container_name],
                            false,
                        ),
                    ));
                    return if let Some(path) = self
                        .failure_log_path_store
                        .as_ref()
                        .and_then(|store| store.take(&run_id))
                    {
                        Ok(response.with_diagnostics_path(path.display().to_string()))
                    } else {
                        Ok(response)
                    };
                }
                Err(error) => {
                    self.events.publish(Event::RunFailed(RunFailedPayload::new(
                        run_id,
                        repository_path.display().to_string(),
                        None,
                        error.to_string(),
                    )));
                    return Err(error);
                }
            }
        }

        let result = (self.action_factory)(container).execute(request);
        match result {
            Ok(response) => {
                self.events
                    .publish(Event::StepFinished(StepFinishedPayload::new(
                        run_id.clone(),
                        StepFinishedDetails::new(
                            "standalone action".to_owned(),
                            "standalone".to_owned(),
                            action_ref,
                            response.exit_code() == 0,
                            Some(response.exit_code()),
                        )
                        .with_stdout(response.stdout().to_owned())
                        .with_stderr(response.stderr().to_owned()),
                    )));
                self.events.publish(Event::WorkflowRunCompleted(
                    WorkflowRunCompletedPayload::new(
                        run_id.clone(),
                        repository_path.display().to_string(),
                        vec![container_name],
                        response.exit_code() == 0,
                    ),
                ));
                if response.exit_code() == 0 {
                    Ok(response)
                } else if let Some(path) = self
                    .failure_log_path_store
                    .as_ref()
                    .and_then(|store| store.take(&run_id))
                {
                    Ok(response.with_diagnostics_path(path.display().to_string()))
                } else {
                    Ok(response)
                }
            }
            Err(error) => {
                self.events.publish(Event::RunFailed(RunFailedPayload::new(
                    run_id,
                    repository_path.display().to_string(),
                    None,
                    error.to_string(),
                )));
                Err(error)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::StandaloneRunActionAdapter;

    #[test]
    fn discovers_repository_dependency_setup_action() {
        let repository = tempdir().expect("temporary repository");
        let action_directory = repository
            .path()
            .join(".forgejo/actions/install-dependencies");
        fs::create_dir_all(&action_directory).expect("action directory");
        fs::write(action_directory.join("action.yml"), "name: dependencies").expect("action file");

        assert_eq!(
            StandaloneRunActionAdapter::dependency_action_ref(
                repository.path(),
                "./.forgejo/actions/ci-validate"
            ),
            Some("./.forgejo/actions/install-dependencies".to_owned())
        );
    }

    #[test]
    fn does_not_prepare_dependency_action_against_itself() {
        let repository = tempdir().expect("temporary repository");
        let action_directory = repository
            .path()
            .join(".forgejo/actions/install-dependencies");
        fs::create_dir_all(&action_directory).expect("action directory");
        fs::write(action_directory.join("action.yml"), "name: dependencies").expect("action file");

        assert_eq!(
            StandaloneRunActionAdapter::dependency_action_ref(
                repository.path(),
                "./.forgejo/actions/install-dependencies"
            ),
            None
        );
    }
}
