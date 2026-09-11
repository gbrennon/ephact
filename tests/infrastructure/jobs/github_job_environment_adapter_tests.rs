#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ephact::application::dtos::requests::BuildJobEnvironmentRequest;
    use ephact::application::ports::outbound::build_job_environment_port::BuildJobEnvironmentPort;
    use ephact::domain::aggregates::Workflow;
    use ephact::infrastructure::jobs::GitHubJobEnvironmentAdapter;
    use ephact::infrastructure::workflows::yaml::WorkflowYaml;

    const DEFAULT_PATH: &str = "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin";

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

    #[test]
    fn execute_lets_the_job_environment_override_the_workflow_one() {
        let workflow = workflow("name: Ci\non: push\nenv:\n  MODE: workflow\njobs: {}\n");

        let response = GitHubJobEnvironmentAdapter::new().execute(BuildJobEnvironmentRequest::new(
            &workflow,
            &job_env(&[("MODE", "job")]),
        ));

        assert_eq!(response.env().get("MODE").map(String::as_str), Some("job"));
    }

    #[test]
    fn execute_sets_the_runners_own_variables() {
        let workflow = workflow("name: Ci\non: push\njobs: {}\n");

        let response = GitHubJobEnvironmentAdapter::new()
            .execute(BuildJobEnvironmentRequest::new(&workflow, &HashMap::new()));

        assert_eq!(
            response.env().get("GITHUB_PATH").map(String::as_str),
            Some("/workspace/.github_path")
        );
        assert_eq!(
            response.env().get("GITHUB_ENV").map(String::as_str),
            Some("/workspace/.github_env")
        );
        assert_eq!(
            response.env().get("GITHUB_WORKSPACE").map(String::as_str),
            Some("/workspace")
        );
    }

    #[test]
    fn execute_defaults_the_path_when_neither_workflow_nor_job_declares_one() {
        let workflow = workflow("name: Ci\non: push\njobs: {}\n");

        let response = GitHubJobEnvironmentAdapter::new()
            .execute(BuildJobEnvironmentRequest::new(&workflow, &HashMap::new()));

        assert_eq!(
            response.env().get("PATH").map(String::as_str),
            Some(DEFAULT_PATH)
        );
    }

    #[test]
    fn execute_keeps_a_declared_path() {
        let workflow = workflow("name: Ci\non: push\nenv:\n  PATH: /custom/bin\njobs: {}\n");

        let response = GitHubJobEnvironmentAdapter::new()
            .execute(BuildJobEnvironmentRequest::new(&workflow, &HashMap::new()));

        assert_eq!(
            response.env().get("PATH").map(String::as_str),
            Some("/custom/bin")
        );
    }

    #[test]
    fn execute_keeps_a_job_declared_path() {
        let workflow = workflow("name: Ci\non: push\njobs: {}\n");

        let response = GitHubJobEnvironmentAdapter::new().execute(BuildJobEnvironmentRequest::new(
            &workflow,
            &job_env(&[("PATH", "/job/bin")]),
        ));

        assert_eq!(
            response.env().get("PATH").map(String::as_str),
            Some("/job/bin")
        );
    }
}
