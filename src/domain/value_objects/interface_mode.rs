#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum InterfaceMode {
    #[default]
    Tui,
    Cli,
}
