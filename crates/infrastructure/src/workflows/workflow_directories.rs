/// Repository-relative directories scanned for workflow files, in lookup order.
pub const WORKFLOW_DIRECTORIES: [&str; 3] =
    [".forgejo/workflows", ".github/workflows", ".woodpecker"];

/// Substring markers mapped to their platform display name, in match order.
const PLATFORM_MARKERS: [(&str, &str); 3] = [
    ("forgejo", "Forgejo"),
    ("github", "GitHub"),
    ("woodpecker", "Woodpecker"),
];

/// Returns the display name of the platform associated with a workflow directory path.
pub fn platform_display_name(directory: &str) -> &'static str {
    PLATFORM_MARKERS
        .iter()
        .find(|(marker, _)| directory.contains(marker))
        .map(|(_, name)| *name)
        .unwrap_or("Unknown")
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
