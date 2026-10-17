/// Configuration for the line prefixes suppressed by [`super::stderr_filter::StderrFilter`].
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StderrFilterConfig {
    suppressed_line_prefixes: Vec<String>,
}

impl Default for StderrFilterConfig {
    fn default() -> Self {
        Self {
            suppressed_line_prefixes: vec![
                "Error: crun: executable file `bash` not found".to_owned(),
                "Error: read unixpacket".to_owned(),
            ],
        }
    }
}

impl StderrFilterConfig {
    /// Creates configuration that suppresses lines starting with the provided
    /// non-empty prefixes.
    pub fn from_prefixes(suppressed_line_prefixes: Vec<String>) -> Self {
        let suppressed_line_prefixes: Vec<String> = suppressed_line_prefixes
            .into_iter()
            .filter(|prefix| !prefix.is_empty())
            .collect();

        Self {
            suppressed_line_prefixes,
        }
    }

    /// Returns the configured prefixes in matching order.
    pub fn suppressed_line_prefixes(&self) -> &[String] {
        &self.suppressed_line_prefixes
    }
}
