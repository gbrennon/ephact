use std::path::Path;

/// Resolves workflow and action source names from parsed names or source files.
pub struct SourceNameResolver;

impl SourceNameResolver {
    /// Prefers a non-empty parsed name and falls back to the source filename.
    pub fn resolve(parsed_name: Option<&str>, source_file: &Path) -> Option<String> {
        parsed_name
            .map(str::trim)
            .filter(|name| !name.is_empty())
            .map(str::to_owned)
            .or_else(|| Self::file_name(source_file))
    }

    /// Returns the UTF-8 filename, preserving its extension.
    pub fn file_name(source_file: &Path) -> Option<String> {
        source_file
            .file_name()
            .and_then(|file_name| file_name.to_str())
            .map(str::to_owned)
    }
}
