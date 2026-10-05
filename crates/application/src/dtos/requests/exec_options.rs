use std::collections::HashMap;

/// Options for executing a command with an optional working directory and environment entries.
pub struct ExecOptions<'a> {
    cmd: &'a [String],
    workdir: Option<&'a str>,
    env: &'a HashMap<String, String>,
}

impl<'a> ExecOptions<'a> {
    /// Creates execution options from a command, optional working directory, and environment entries.
    pub fn new(
        cmd: &'a [String],
        workdir: Option<&'a str>,
        env: &'a HashMap<String, String>,
    ) -> Self {
        Self { cmd, workdir, env }
    }

    /// Returns the command arguments.
    pub fn cmd(&self) -> &[String] {
        self.cmd
    }

    /// Returns the optional working directory.
    pub fn workdir(&self) -> Option<&str> {
        self.workdir
    }

    /// Returns the environment entries.
    pub fn env(&self) -> &HashMap<String, String> {
        self.env
    }
}
