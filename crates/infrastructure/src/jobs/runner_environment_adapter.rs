use std::collections::HashMap;

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
    fn insert_environment(
        env: HashMap<String, String>,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> HashMap<String, String> {
        env.into_iter()
            .chain(std::iter::once((key.into(), value.into())))
            .collect()
    }

    fn insert_default_environment(
        env: HashMap<String, String>,
        key: impl Into<String>,
        value: impl Into<String>,
    ) -> HashMap<String, String> {
        let key = key.into();
        if env.contains_key(&key) {
            return env;
        }
        Self::insert_environment(env, key, value)
    }

    fn insert_event_environment(
        env: HashMap<String, String>,
        context: &EvaluationContext,
    ) -> HashMap<String, String> {
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
        let env = values.into_iter().fold(env, |env, (suffix, key)| {
            let value = context
                .get("github")
                .and_then(|github| github.property(key))
                .and_then(|value| value.as_text())
                .unwrap_or_default()
                .to_owned();
            let env = Self::insert_environment(env, format!("GITHUB_{suffix}"), value.clone());
            Self::insert_environment(env, format!("FORGEJO_{suffix}"), value)
        });
        let env = Self::insert_environment(env, "GITHUB_EVENT_PATH", RUNNER_EVENT_FILE);
        Self::insert_environment(env, "FORGEJO_EVENT_PATH", RUNNER_EVENT_FILE)
    }

    fn add_event_environment(
        env: HashMap<String, String>,
        request: &BuildJobEnvironmentRequest,
    ) -> HashMap<String, String> {
        if request.event_payload().is_some() {
            return Self::insert_event_environment(env, request.context());
        }
        env
    }
}

impl Default for RunnerEnvironmentAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl JobEnvironmentBuilderPort for RunnerEnvironmentAdapter {
    fn build(&self, request: BuildJobEnvironmentRequest) -> BuildJobEnvironmentResponse {
        let env = request
            .workflow()
            .env()
            .clone()
            .into_iter()
            .chain(
                request
                    .job_env()
                    .iter()
                    .map(|(key, value)| (key.clone(), value.clone())),
            )
            .collect();
        let env = Self::add_event_environment(env, &request);
        let env = Self::insert_environment(env, "GITHUB_PATH", RUNNER_PATH_FILE);
        let env = Self::insert_environment(env, "GITHUB_ENV", RUNNER_ENV_FILE);
        let env = Self::insert_environment(env, "GITHUB_OUTPUT", RUNNER_OUTPUT_FILE);
        let env = Self::insert_environment(env, "GITHUB_WORKSPACE", CONTAINER_WORKSPACE);
        let env = Self::insert_default_environment(env, "RUNNER_TOOL_CACHE", RUNNER_TOOL_CACHE);
        let env = Self::insert_default_environment(env, "RUNNER_TEMP", "/tmp");
        let env = Self::insert_default_environment(env, "PATH", DEFAULT_PATH);
        BuildJobEnvironmentResponse::new(env)
    }
}
