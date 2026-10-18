#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ephact::{
        application::{
            dtos::requests::BuildJobEnvironmentRequest,
            ports::outbound::job_environment_builder_port::JobEnvironmentBuilderPort,
        },
        domain::{
            aggregates::Workflow,
            value_objects::{ContextValue, EvaluationContext},
        },
        infrastructure::{jobs::RunnerEnvironmentAdapter, workflows::actions::WorkflowYaml},
    };

    const DEFAULT_PATH: &str = concat!(
        "/opt/acttoolcache/node/24.19.0/x64/bin:",
        "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin",
    );

    fn workflow(yaml: &str) -> Workflow {
        serde_yaml::from_str::<WorkflowYaml>(yaml)
            .unwrap()
            .into_domain()
    }

    fn job_env(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect()
    }
    fn event_context() -> EvaluationContext {
        let event = ContextValue::mapping([
            ("ref".into(), ContextValue::text("refs/heads/main")),
            ("after".into(), ContextValue::text("abc123")),
            (
                "pull_request".into(),
                ContextValue::mapping([
                    (
                        "head".into(),
                        ContextValue::mapping([("ref".into(), ContextValue::text("feature"))]),
                    ),
                    (
                        "base".into(),
                        ContextValue::mapping([("ref".into(), ContextValue::text("main"))]),
                    ),
                ]),
            ),
            (
                "sender".into(),
                ContextValue::mapping([("login".into(), ContextValue::text("octocat"))]),
            ),
            (
                "repository".into(),
                ContextValue::mapping([(
                    "full_name".into(),
                    ContextValue::text("octocat/example"),
                )]),
            ),
        ]);
        EvaluationContext::new().with_root(
            "github",
            ContextValue::mapping([
                ("event_name".into(), ContextValue::text("pull_request")),
                ("ref".into(), event.property("ref").unwrap().clone()),
                ("ref_name".into(), ContextValue::text("main")),
                ("head_ref".into(), ContextValue::text("feature")),
                ("base_ref".into(), ContextValue::text("main")),
                ("sha".into(), ContextValue::text("abc123")),
                ("actor".into(), ContextValue::text("octocat")),
                ("repository".into(), ContextValue::text("octocat/example")),
                ("repository_owner".into(), ContextValue::text("octocat")),
                ("workflow".into(), ContextValue::text("CI")),
                ("job".into(), ContextValue::text("build")),
                ("run_id".into(), ContextValue::text("run-1")),
                ("event".into(), event),
            ]),
        )
    }

    #[test]
    fn execute_lets_the_job_environment_override_the_workflow_one() {
        let workflow = workflow("name: Ci\non: push\nenv:\n  MODE: workflow\njobs: {}\n");

        let response = RunnerEnvironmentAdapter::new().build(BuildJobEnvironmentRequest::new(
            workflow.clone(),
            job_env(&[("MODE", "job")]),
        ));

        assert_eq!(response.env().get("MODE").map(String::as_str), Some("job"));
    }

    #[test]
    fn execute_sets_the_runners_own_variables() {
        let workflow = workflow("name: Ci\non: push\njobs: {}\n");

        let response = RunnerEnvironmentAdapter::new().build(BuildJobEnvironmentRequest::new(
            workflow.clone(),
            HashMap::new(),
        ));

        assert_eq!(
            response.env().get("GITHUB_PATH").map(String::as_str),
            Some("/tmp/.ephact_path")
        );
        assert_eq!(
            response.env().get("GITHUB_ENV").map(String::as_str),
            Some("/tmp/.ephact_env")
        );
        assert_eq!(
            response.env().get("GITHUB_OUTPUT").map(String::as_str),
            Some("/tmp/.ephact_output")
        );
        assert_eq!(
            response.env().get("GITHUB_WORKSPACE").map(String::as_str),
            Some("/workspace")
        );
    }

    #[test]
    fn execute_defaults_the_path_when_neither_workflow_nor_job_declares_one() {
        let workflow = workflow("name: Ci\non: push\njobs: {}\n");

        let response = RunnerEnvironmentAdapter::new().build(BuildJobEnvironmentRequest::new(
            workflow.clone(),
            HashMap::new(),
        ));

        assert_eq!(
            response.env().get("PATH").map(String::as_str),
            Some(DEFAULT_PATH)
        );
    }

    #[test]
    fn execute_keeps_a_declared_path() {
        let workflow = workflow("name: Ci\non: push\nenv:\n  PATH: /custom/bin\njobs: {}\n");

        let response = RunnerEnvironmentAdapter::new().build(BuildJobEnvironmentRequest::new(
            workflow.clone(),
            HashMap::new(),
        ));

        assert_eq!(
            response.env().get("PATH").map(String::as_str),
            Some("/custom/bin")
        );
    }

    #[test]
    fn execute_keeps_a_job_declared_path() {
        let workflow = workflow("name: Ci\non: push\njobs: {}\n");

        let response = RunnerEnvironmentAdapter::new().build(BuildJobEnvironmentRequest::new(
            workflow.clone(),
            job_env(&[("PATH", "/job/bin")]),
        ));

        assert_eq!(
            response.env().get("PATH").map(String::as_str),
            Some("/job/bin")
        );
    }
    #[test]
    fn execute_injects_github_and_forgejo_event_variables_from_context() {
        let workflow = workflow("name: Ci\non: pull_request\njobs: {}\n");
        let response = RunnerEnvironmentAdapter::new().build(
            BuildJobEnvironmentRequest::new(workflow, HashMap::new())
                .with_context(event_context())
                .with_event_payload_opt(Some(r#"{"event":"pull_request"}"#)),
        );

        let expected = [
            ("EVENT_NAME", "pull_request"),
            ("REF", "refs/heads/main"),
            ("REF_NAME", "main"),
            ("HEAD_REF", "feature"),
            ("BASE_REF", "main"),
            ("SHA", "abc123"),
            ("ACTOR", "octocat"),
            ("REPOSITORY", "octocat/example"),
            ("REPOSITORY_OWNER", "octocat"),
            ("WORKFLOW", "CI"),
            ("JOB", "build"),
            ("RUN_ID", "run-1"),
        ];
        for (suffix, value) in expected {
            assert_eq!(
                response
                    .env()
                    .get(&format!("GITHUB_{suffix}"))
                    .map(String::as_str),
                Some(value)
            );
            assert_eq!(
                response
                    .env()
                    .get(&format!("FORGEJO_{suffix}"))
                    .map(String::as_str),
                Some(value)
            );
        }
        assert_eq!(
            response.env().get("GITHUB_EVENT_PATH").map(String::as_str),
            Some("/workspace/.github/event.json")
        );
        assert_eq!(
            response.env().get("FORGEJO_EVENT_PATH").map(String::as_str),
            Some("/workspace/.github/event.json")
        );
    }

    #[test]
    fn execute_does_not_inject_event_variables_without_event_context() {
        let workflow = workflow("name: Ci\non: push\njobs: {}\n");
        let response = RunnerEnvironmentAdapter::new()
            .build(BuildJobEnvironmentRequest::new(workflow, HashMap::new()));

        for prefix in ["GITHUB", "FORGEJO"] {
            for suffix in [
                "EVENT_NAME",
                "EVENT_PATH",
                "REF",
                "REF_NAME",
                "HEAD_REF",
                "BASE_REF",
                "SHA",
                "ACTOR",
                "REPOSITORY",
                "REPOSITORY_OWNER",
                "WORKFLOW",
                "JOB",
                "RUN_ID",
            ] {
                assert!(!response.env().contains_key(&format!("{prefix}_{suffix}")));
            }
        }
    }
}
