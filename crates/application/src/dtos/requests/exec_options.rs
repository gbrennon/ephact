use std::collections::HashMap;

pub struct ExecOptions<'a> {
    cmd: &'a [String],
    workdir: Option<&'a str>,
    env: &'a HashMap<String, String>,
}

impl<'a> ExecOptions<'a> {
    pub fn new(
        cmd: &'a [String],
        workdir: Option<&'a str>,
        env: &'a HashMap<String, String>,
    ) -> Self {
        Self { cmd, workdir, env }
    }

    pub fn cmd(&self) -> &[String] {
        self.cmd
    }

    pub fn workdir(&self) -> Option<&str> {
        self.workdir
    }

    pub fn env(&self) -> &HashMap<String, String> {
        self.env
    }
}
