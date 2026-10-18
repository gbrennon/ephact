use std::{error::Error, time::Instant};

use crate::{
    application::dtos::requests::RunWorkflowRequest,
    domain::{Repository, value_objects::workflow_run_config::WorkflowRunConfig},
    repositories::RepositoryResolver,
    workflows::execution::workflow_run_config_mapper::WorkflowRunConfigMapper,
};

pub(crate) struct RunExecutionContext {
    repository: Repository,
    repository_path: String,
    config: WorkflowRunConfig,
    run_id: String,
    workflow_name: Option<String>,
    started_at: Instant,
}

impl RunExecutionContext {
    pub(crate) fn new(request: RunWorkflowRequest) -> Result<Self, Box<dyn Error>> {
        let repository_path = request.repository_path().to_path_buf();
        let repository = RepositoryResolver::resolve(
            repository_path.clone(),
            request.repository_name().to_owned(),
        )
        .map_err(|error| format!("{error}"))?;
        let config = WorkflowRunConfigMapper::default()
            .with_workflow(request.workflow().map(str::to_string))
            .with_job(request.job().map(str::to_string))
            .with_event(request.event().map(str::to_string))
            .with_event_path(request.event_path().map(ToOwned::to_owned))
            .with_workflow_directory(request.workflow_directory().map(ToOwned::to_owned))
            .with_inputs(request.inputs().to_vec())
            .with_secrets(request.secrets().to_vec())
            .with_all_workflows(request.all_workflows())
            .with_allow_repo_writes(request.allow_repo_writes())
            .with_allow_real_container(request.allow_real_container())
            .with_allow_real_fetcher(request.allow_real_fetcher())
            .with_allow_network(request.allow_network())
            .into_config()?;
        let run_id = request.run_id().to_string();
        let workflow_name = config
            .workflow()
            .map(|workflow| workflow.as_str().to_string());
        Ok(Self {
            repository,
            repository_path: repository_path.display().to_string(),
            config,
            run_id,
            workflow_name,
            started_at: Instant::now(),
        })
    }
}
impl RunExecutionContext {
    pub(crate) fn repository(&self) -> &Repository {
        &self.repository
    }

    pub(crate) fn repository_path(&self) -> &str {
        &self.repository_path
    }

    pub(crate) fn config(&self) -> &WorkflowRunConfig {
        &self.config
    }

    pub(crate) fn run_id(&self) -> &str {
        &self.run_id
    }

    pub(crate) fn workflow_name(&self) -> Option<&String> {
        self.workflow_name.as_ref()
    }

    pub(crate) fn workflow_name_mut(&mut self) -> &mut Option<String> {
        &mut self.workflow_name
    }

    pub(crate) fn started_at(&self) -> Instant {
        self.started_at
    }
}
