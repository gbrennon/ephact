use crate::{
    application::{
        dtos::{requests::BuildJobEnvironmentRequest, responses::BuildJobEnvironmentResponse},
        ports::outbound::job_environment_builder_port::JobEnvironmentBuilderPort,
    },
    infrastructure::containers::workspace::{
        CONTAINER_WORKSPACE, RUNNER_ENV_FILE, RUNNER_PATH_FILE,
    },
};

/// `PATH` a job runs with when neither the workflow nor the job declares one.
const DEFAULT_PATH: &str = "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin";

/// Infrastructure adapter that builds the execution environment for a job's container
/// following the GitHub Actions specification: merging workflow and job environments,
/// and setting `GITHUB_PATH`, `GITHUB_ENV`, `GITHUB_WORKSPACE`, and default `PATH`.
pub struct RunnerEnvironmentAdapter;

impl RunnerEnvironmentAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for RunnerEnvironmentAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl JobEnvironmentBuilderPort for RunnerEnvironmentAdapter {
    fn build(&self, request: BuildJobEnvironmentRequest) -> BuildJobEnvironmentResponse {
        let mut env = request.workflow().env().clone();
        for (key, value) in request.job_env() {
            env.insert(key.clone(), value.clone());
        }

        env.insert("GITHUB_PATH".into(), RUNNER_PATH_FILE.into());
        env.insert("GITHUB_ENV".into(), RUNNER_ENV_FILE.into());
        env.insert("GITHUB_WORKSPACE".into(), CONTAINER_WORKSPACE.into());
        env.entry("PATH".to_string())
            .or_insert_with(|| DEFAULT_PATH.to_string());
        BuildJobEnvironmentResponse::new(env)
    }
}
