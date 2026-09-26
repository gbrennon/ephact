/// Repository-relative directories scanned for workflow files, in lookup order.
pub const WORKFLOW_DIRECTORIES: [&str; 2] = [".forgejo/workflows", ".github/workflows"];

/// Returns the display name of the platform associated with a workflow directory path.
pub fn platform_display_name(directory: &str) -> &'static str {
    if directory.contains("forgejo") {
        "Forgejo"
    } else if directory.contains("github") {
        "GitHub"
    } else {
        "Unknown"
    }
}

/// Returns a comma-separated list of supported platform display names.
pub fn supported_platforms_display() -> String {
    WORKFLOW_DIRECTORIES
        .iter()
        .map(|directory| platform_display_name(directory))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Returns a comma-separated list of supported workflow directory paths.
pub fn supported_workflows_display() -> String {
    WORKFLOW_DIRECTORIES.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn platform_display_name_identifies_known_platforms() {
        assert_eq!(platform_display_name(".forgejo/workflows"), "Forgejo");
        assert_eq!(platform_display_name(".github/workflows"), "GitHub");
        assert_eq!(platform_display_name(".other/workflows"), "Unknown");
    }

    #[test]
    fn supported_platforms_display_lists_all_platforms() {
        let display = supported_platforms_display();
        assert!(display.contains("Forgejo"));
        assert!(display.contains("GitHub"));
    }

    #[test]
    fn supported_workflows_display_lists_all_directories() {
        let display = supported_workflows_display();
        assert!(display.contains(".forgejo/workflows"));
        assert!(display.contains(".github/workflows"));
    }
}
