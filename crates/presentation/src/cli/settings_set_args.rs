use clap::{Args, Subcommand, ValueEnum};

#[derive(Subcommand)]
#[command(
    about = "Manage persisted settings.",
    long_about = r#"Manage persisted settings.

show reads and displays the current persisted settings.
set <name> <value> changes exactly one setting after reading the current settings and preserves the other settings.
Boolean settings accept only true or false.
default-interface accepts tui or cli; navigation-mode accepts standard, vim, or emacs.
failure-log-retention-hours requires a positive integer greater than zero.
marker accepts one of the existing preset marker strings or arbitrary custom text.
forward-ssh is persisted in the host settings store.
reset restores all persisted settings to safe defaults including forward-ssh = false."#
)]
pub enum SettingsCommand {
    #[command(
        about = "Display the current persisted settings.",
        long_about = "show reads and displays the current persisted settings."
    )]
    Show,
    #[command(
        about = "Change one persisted setting.",
        long_about = "set <name> <value> reads the current settings, changes exactly one setting, and preserves the other settings."
    )]
    Set(SettingsSetArgs),
    #[command(
        about = "Restore safe persisted defaults.",
        long_about = "reset restores all persisted settings to safe defaults including forward-ssh = false."
    )]
    Reset,
}

#[derive(Args)]
pub struct SettingsSetArgs {
    #[arg(value_name = "NAME", help = "Name of the setting to change.")]
    name: SettingName,
    #[arg(
        allow_hyphen_values = true,
        value_name = "VALUE",
        help = "Value to persist for the selected setting."
    )]
    value: String,
}

impl SettingsSetArgs {
    pub fn name(&self) -> SettingName {
        self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub enum SettingName {
    DefaultInterface,
    AllowRepoWrites,
    AllowRealContainer,
    AllowRealFetcher,
    AllowNetwork,
    Preserve,
    Verbose,
    Interactive,
    AllWorkflows,
    FailureLogRetentionHours,
    NavigationMode,
    ForwardSsh,
    Marker,
}
