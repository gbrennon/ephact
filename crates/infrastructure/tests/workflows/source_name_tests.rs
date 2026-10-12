#[cfg(test)]
#[path = "../../src/workflows/source_name.rs"]
mod source_name;

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::source_name::SourceNameResolver;

    #[test]
    fn resolve_source_name_prefers_a_non_empty_parsed_name() {
        assert_eq!(
            SourceNameResolver::resolve(
                Some("  Build workflow  "),
                Path::new(".forgejo/workflows/build.yml"),
            ),
            Some("Build workflow".to_string())
        );
    }

    #[test]
    fn resolve_source_name_uses_the_filename_when_the_name_is_missing() {
        assert_eq!(
            SourceNameResolver::resolve(None, Path::new(".forgejo/workflows/build.yml")),
            Some("build.yml".to_string())
        );
    }

    #[test]
    fn resolve_source_name_uses_the_filename_for_whitespace_only_names() {
        assert_eq!(
            SourceNameResolver::resolve(Some("  \t"), Path::new(".forgejo/workflows/build.yml"),),
            Some("build.yml".to_string())
        );
    }

    #[test]
    fn source_file_name_keeps_the_extension() {
        assert_eq!(
            SourceNameResolver::file_name(Path::new(".forgejo/workflows/build.yaml")),
            Some("build.yaml".to_string())
        );
    }
}
