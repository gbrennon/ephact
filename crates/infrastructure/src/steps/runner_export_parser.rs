use std::collections::HashMap;

type Record = (String, String);
type MultilineRecord = (String, String, Vec<String>);
type ParseState = (Vec<Record>, Option<MultilineRecord>);

/// Parses key/value records written to a runner export file.
pub struct RunnerExportParser;

impl RunnerExportParser {
    /// Parses single-line and documented multiline export records.
    pub fn parse(contents: &str) -> HashMap<String, String> {
        let (records, _) = contents
            .lines()
            .fold((Vec::new(), None), Self::consume_line);
        records.into_iter().collect()
    }

    fn consume_line((records, multiline): ParseState, line: &str) -> ParseState {
        match multiline {
            Some((name, delimiter, lines)) if line == delimiter => (
                records
                    .into_iter()
                    .chain(std::iter::once((name, lines.join("\n"))))
                    .collect(),
                None,
            ),
            Some((name, delimiter, lines)) => (
                records,
                Some((
                    name,
                    delimiter,
                    lines
                        .into_iter()
                        .chain(std::iter::once(line.to_owned()))
                        .collect(),
                )),
            ),
            None => Self::start_record(records, line),
        }
    }

    fn start_record(records: Vec<Record>, line: &str) -> ParseState {
        match line.split_once("<<") {
            Some((name, delimiter)) if !name.is_empty() && !delimiter.is_empty() => (
                records,
                Some((name.to_owned(), delimiter.to_owned(), Vec::new())),
            ),
            Some(_) => (records, None),
            None => match Self::parse_single_line(line) {
                Some(record) => (
                    records.into_iter().chain(std::iter::once(record)).collect(),
                    None,
                ),
                None => (records, None),
            },
        }
    }

    fn parse_single_line(line: &str) -> Option<Record> {
        let (name, value) = line.split_once('=')?;
        (!name.is_empty()).then(|| (name.to_owned(), value.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::RunnerExportParser;

    #[test]
    fn parses_single_line_records() {
        assert_eq!(
            RunnerExportParser::parse("A=1\nQUERY=a=b=c\n"),
            [
                ("A".to_owned(), "1".to_owned()),
                ("QUERY".to_owned(), "a=b=c".to_owned()),
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn parses_documented_multiline_records() {
        assert_eq!(
            RunnerExportParser::parse("DESCRIPTION<<EOF\nfirst\nsecond\nEOF\n"),
            [("DESCRIPTION".to_owned(), "first\nsecond".to_owned())]
                .into_iter()
                .collect()
        );
    }

    #[test]
    fn ignores_unclosed_multiline_records() {
        assert!(RunnerExportParser::parse("DESCRIPTION<<EOF\nfirst\n").is_empty());
    }

    #[test]
    fn ignores_records_with_empty_names_or_delimiters() {
        assert_eq!(
            RunnerExportParser::parse(
                "=ignored\n<<EOF\nignored\nEOF\nEMPTY<<\nignored\nKEPT=value\n",
            ),
            [("KEPT".to_owned(), "value".to_owned())]
                .into_iter()
                .collect()
        );
    }

    #[test]
    fn parses_empty_multiline_values() {
        assert_eq!(
            RunnerExportParser::parse("EMPTY<<EOF\nEOF\n"),
            [("EMPTY".to_owned(), String::new())].into_iter().collect()
        );
    }

    #[test]
    fn later_records_replace_earlier_records_with_the_same_name() {
        assert_eq!(
            RunnerExportParser::parse("VALUE=first\nVALUE=second\n"),
            [("VALUE".to_owned(), "second".to_owned())]
                .into_iter()
                .collect()
        );
    }
}
