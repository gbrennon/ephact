use std::{error::Error, time::Instant};

use crate::{
    application::dtos::requests::RunWorkflowRequest,
    domain::{Repository, value_objects::workflow_run_config::WorkflowRunConfig},
    repositories::RepositoryResolver,
    workflows::execution::workflow_run_config_mapper::WorkflowRunConfigMapper,
};

pub(crate) struct RunExecutionContext {
    pub(crate) repository: Repository,
    pub(crate) repository_path: String,
    pub(crate) config: WorkflowRunConfig,
    pub(crate) run_id: String,
    pub(crate) workflow_name: Option<String>,
    pub(crate) started_at: Instant,
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
