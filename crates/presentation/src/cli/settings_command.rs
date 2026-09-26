use clap::{Args, Subcommand, ValueEnum};

#[derive(Subcommand)]
pub enum SettingsCommand {
    Show,
    Set(SettingsSetArgs),
    Reset,
}

#[derive(Args)]
pub struct SettingsSetArgs {
    name: SettingName,
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
}
