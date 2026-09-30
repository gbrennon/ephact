use std::path::Path;

pub(crate) fn resolve_source_name(parsed_name: Option<&str>, source_file: &Path) -> Option<String> {
    parsed_name
        .map(str::trim)
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .or_else(|| source_file_name(source_file))
}

pub(crate) fn source_file_name(source_file: &Path) -> Option<String> {
    source_file
        .file_name()
        .and_then(|file_name| file_name.to_str())
        .map(str::to_owned)
}
