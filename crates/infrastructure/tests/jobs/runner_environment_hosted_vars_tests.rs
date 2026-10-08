#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use ephact::{
        application::{
            dtos::requests::BuildJobEnvironmentRequest,
            ports::outbound::job_environment_builder_port::JobEnvironmentBuilderPort,
        },
        domain::aggregates::Workflow,
        infrastructure::jobs::RunnerEnvironmentAdapter,
    };

    #[test]
    fn hosted_runner_actions_receive_tool_cache_and_temp_paths() {
        let workflow = Workflow::new(
            Some("ComChan".to_string()),
            Vec::new(),
            HashMap::new(),
            HashMap::new(),
        );

        let environment = RunnerEnvironmentAdapter::new()
            .build(BuildJobEnvironmentRequest::new(workflow, HashMap::new()));

        assert_eq!(
            environment
                .env()
                .get("RUNNER_TOOL_CACHE")
                .map(String::as_str),
            Some("/opt/hostedtoolcache")
        );
        assert_eq!(
            environment.env().get("RUNNER_TEMP").map(String::as_str),
            Some("/tmp")
        );
    }
}
