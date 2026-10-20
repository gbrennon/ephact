#[cfg(test)]
mod tests {
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
        let container = container("artifact=ready\nQUERY=a=b=c\n");

        let outputs = ReadStepOutputExportsService::new()
            .read(ReadStepOutputExportsRequest::new(), &container);

        assert_eq!(outputs.get("artifact").map(String::as_str), Some("ready"));
        assert_eq!(outputs.get("QUERY").map(String::as_str), Some("a=b=c"));
    }

    #[test]
    fn execute_parses_documented_multiline_outputs() {
        let container = container("REPORT<<EOF\nfirst line\nsecond line\nEOF\n");

        let outputs = ReadStepOutputExportsService::new()
            .read(ReadStepOutputExportsRequest::new(), &container);

        assert_eq!(
            outputs.get("REPORT").map(String::as_str),
            Some("first line\nsecond line"),
        );
    }

    #[test]
    fn execute_returns_no_outputs_when_the_file_was_never_written() {
        let container = StubExportingContainer::empty();

        let outputs = ReadStepOutputExportsService::new()
            .read(ReadStepOutputExportsRequest::new(), &container);

        assert!(outputs.is_empty());
    }
}
