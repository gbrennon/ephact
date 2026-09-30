use ephact::infrastructure::workflows::workflow_directories::{
    platform_display_name, supported_platforms_display, supported_workflows_display,
};

#[test]
fn platform_display_name_identifies_known_platforms() {
    assert_eq!(platform_display_name(".forgejo/workflows"), "Forgejo");
    assert_eq!(platform_display_name(".github/workflows"), "GitHub");
    assert_eq!(platform_display_name(".woodpecker"), "Woodpecker");
    assert_eq!(platform_display_name(".other/workflows"), "Unknown");
}

#[test]
fn supported_platforms_display_lists_all_platforms() {
    let display = supported_platforms_display();

    assert!(display.contains("Forgejo"));
    assert!(display.contains("GitHub"));
    assert!(display.contains("Woodpecker"));
}

#[test]
fn supported_workflows_display_lists_all_directories() {
    let display = supported_workflows_display();

    assert!(display.contains(".forgejo/workflows"));
    assert!(display.contains(".github/workflows"));
    assert!(display.contains(".woodpecker"));
}
