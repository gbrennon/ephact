use ephact::{
    application::dtos::requests::ReadStepEnvExportsRequest,
    infrastructure::steps::read_step_env_exports_service::ReadStepEnvExportsService,
};
use ephact_application::ports::outbound::ReadStepEnvExportsPort;

use crate::common::fakes::stub_exporting_container::StubExportingContainer;

fn container(contents: &str) -> StubExportingContainer {
    StubExportingContainer::holding(vec![("/tmp/.ephact_env".to_string(), contents.to_string())])
}

#[test]
fn execute_returns_the_assignments_and_skips_lines_without_an_equals() {
    let container = container("A=1\nnot-an-assignment\nB=2\n");

    let env = ReadStepEnvExportsService::new().read(ReadStepEnvExportsRequest::new(), &container);

    assert_eq!(env.len(), 2);
    assert_eq!(env.get("A").map(String::as_str), Some("1"));
    assert_eq!(env.get("B").map(String::as_str), Some("2"));
}

#[test]
fn execute_keeps_everything_after_the_first_equals_in_the_value() {
    let container = container("QUERY=a=b=c\n");

    let env = ReadStepEnvExportsService::new().read(ReadStepEnvExportsRequest::new(), &container);

    assert_eq!(env.get("QUERY").map(String::as_str), Some("a=b=c"));
}

#[test]
fn execute_returns_no_variables_when_the_file_was_never_written() {
    let container = StubExportingContainer::empty();

    let env = ReadStepEnvExportsService::new().read(ReadStepEnvExportsRequest::new(), &container);

    assert!(env.is_empty());
}

#[test]
fn execute_parses_documented_multiline_values() {
    let container = container("DESCRIPTION<<EOF\nfirst line\nsecond line\nEOF\n");

    let env = ReadStepEnvExportsService::new().read(ReadStepEnvExportsRequest::new(), &container);

    assert_eq!(
        env.get("DESCRIPTION").map(String::as_str),
        Some("first line\nsecond line"),
    );
}

#[test]
fn execute_rejects_protected_environment_variables() {
    let container = container(
        "GITHUB_WORKSPACE=/wrong\nRUNNER_OS=Windows\nNODE_OPTIONS=--require=bad\nCI=false\nMODE=release\n",
    );

    let env = ReadStepEnvExportsService::new().read(ReadStepEnvExportsRequest::new(), &container);

    assert!(!env.contains_key("GITHUB_WORKSPACE"));
    assert!(!env.contains_key("RUNNER_OS"));
    assert!(!env.contains_key("NODE_OPTIONS"));
    assert_eq!(env.get("CI").map(String::as_str), Some("false"));
    assert_eq!(env.get("MODE").map(String::as_str), Some("release"));
}
