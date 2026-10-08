use crate::{
    application::{
        dtos::{requests::BuildJobEnvironmentRequest, responses::BuildJobEnvironmentResponse},
        ports::outbound::job_environment_builder_port::JobEnvironmentBuilderPort,
    },
    containers::workspace::{
        CONTAINER_WORKSPACE, RUNNER_ENV_FILE, RUNNER_OUTPUT_FILE, RUNNER_PATH_FILE,
    },
};

/// `PATH` a job runs with when neither the workflow nor the job declares one.
const DEFAULT_PATH: &str = concat!(
    "/opt/acttoolcache/node/24.19.0/x64/bin:",
    "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
);

/// Directory where hosted-runner actions install tools.
const RUNNER_TOOL_CACHE: &str = "/opt/hostedtoolcache";

/// Infrastructure adapter that builds the execution environment for a job's container
/// following the GitHub Actions specification: merging workflow and job environments,
/// and setting runner paths, tool-cache, and temporary directory variables.
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
        env.insert("GITHUB_OUTPUT".into(), RUNNER_OUTPUT_FILE.into());
        env.insert("GITHUB_WORKSPACE".into(), CONTAINER_WORKSPACE.into());
        env.entry("RUNNER_TOOL_CACHE".into())
            .or_insert_with(|| RUNNER_TOOL_CACHE.to_string());
        env.entry("RUNNER_TEMP".into())
            .or_insert_with(|| "/tmp".to_string());
        env.entry("PATH".to_string())
            .or_insert_with(|| DEFAULT_PATH.to_string());
        BuildJobEnvironmentResponse::new(env)
    }
}
