use ephact::{
    application::dtos::requests::ReadStepOutputExportsRequest,
    infrastructure::steps::read_step_output_exports_service::ReadStepOutputExportsService,
};
use ephact_application::ports::outbound::ReadStepOutputExportsPort;

use crate::common::fakes::stub_exporting_container::StubExportingContainer;

fn container(contents: &str) -> StubExportingContainer {
    StubExportingContainer::holding(vec![(
        "/tmp/.ephact_output".to_string(),
        contents.to_string(),
    )])
}

#[test]
fn execute_returns_single_line_outputs() {
    let container = container("artifact=ready\n");

    let outputs =
        ReadStepOutputExportsService::new().read(ReadStepOutputExportsRequest::new(), &container);

    assert_eq!(outputs.get("artifact").map(String::as_str), Some("ready"));
}

#[test]
fn execute_parses_documented_multiline_outputs() {
    let container = container("REPORT<<EOF\nfirst line\nsecond line\nEOF\n");

    let outputs =
        ReadStepOutputExportsService::new().read(ReadStepOutputExportsRequest::new(), &container);

    assert_eq!(
        outputs.get("REPORT").map(String::as_str),
        Some("first line\nsecond line"),
    );
}
