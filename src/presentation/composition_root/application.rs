use super::super::cli::Cli;

/// Fully-wired presentation layer, returned by [`super::CompositionRoot::compose`].
pub struct Application {
    cli: Cli,
}

impl Application {
    pub fn new(cli: Cli) -> Self {
        Self { cli }
    }

    pub fn cli(&self) -> &Cli {
        &self.cli
    }

    pub fn run<I, T>(self, args: I) -> Result<(), Box<dyn std::error::Error>>
    where
        I: IntoIterator<Item = T>,
        T: Into<std::ffi::OsString> + Clone,
    {
        self.cli.run(args)
    }
}

impl AsRef<Cli> for Application {
    fn as_ref(&self) -> &Cli {
        &self.cli
    }
}

impl std::ops::Deref for Application {
    type Target = Cli;

    fn deref(&self) -> &Self::Target {
        &self.cli
    }
}
