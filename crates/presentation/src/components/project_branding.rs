/// Shared project branding content used by the CLI banner and TUI splash.
pub struct ProjectBranding;

impl ProjectBranding {
    /// Project wordmark shown below the emblem.
    pub const TITLE: &'static str = "EPHACT";
    /// Project description shown below the wordmark.
    pub const SUBTITLE: &'static str = "run workflows locally in containers";
    /// TUI splash continuation hint.
    pub const TUI_HINT: &'static str = "press any key to continue";
    /// CLI usage hint that does not imply an interactive pre-screen.
    pub const CLI_HINT: &'static str = "run ephact --help for usage";
}
