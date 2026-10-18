use crate::{
    application::{
        dtos::{requests::BuildJobEnvironmentRequest, responses::BuildJobEnvironmentResponse},
        ports::outbound::job_environment_builder_port::JobEnvironmentBuilderPort,
    },
    containers::workspace::{
        CONTAINER_WORKSPACE, RUNNER_ENV_FILE, RUNNER_EVENT_FILE, RUNNER_OUTPUT_FILE,
        RUNNER_PATH_FILE,
    },
    domain::value_objects::EvaluationContext,
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
    fn insert_event_environment(
        env: &mut std::collections::HashMap<String, String>,
        context: &EvaluationContext,
    ) {
        let values = [
            ("EVENT_NAME", "event_name"),
            ("REF", "ref"),
            ("REF_NAME", "ref_name"),
            ("HEAD_REF", "head_ref"),
            ("BASE_REF", "base_ref"),
            ("SHA", "sha"),
            ("ACTOR", "actor"),
            ("REPOSITORY", "repository"),
            ("REPOSITORY_OWNER", "repository_owner"),
            ("WORKFLOW", "workflow"),
            ("JOB", "job"),
            ("RUN_ID", "run_id"),
        ];
        for (suffix, key) in values {
            let value = context
                .get("github")
                .and_then(|github| github.property(key))
                .and_then(|value| value.as_text())
                .unwrap_or_default()
                .to_owned();
            env.insert(format!("GITHUB_{suffix}"), value.clone());
            env.insert(format!("FORGEJO_{suffix}"), value);
        }
        env.insert("GITHUB_EVENT_PATH".into(), RUNNER_EVENT_FILE.into());
        env.insert("FORGEJO_EVENT_PATH".into(), RUNNER_EVENT_FILE.into());
    }
    fn add_event_environment(
        env: &mut std::collections::HashMap<String, String>,
        request: &BuildJobEnvironmentRequest,
    ) {
        if request.event_payload().is_some() {
            Self::insert_event_environment(env, request.context());
        }
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
        Self::add_event_environment(&mut env, &request);
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
