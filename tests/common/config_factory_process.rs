use std::{
    path::Path,
    process::{Command, Output},
};

pub fn run_with_home(home: &Path, args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_ephact"))
        .args(args)
        .env("HOME", home)
        .output()
        .expect("ephact should start")
}

pub fn run_without_home(args: &[&str]) -> Output {
    let mut command = Command::new(env!("CARGO_BIN_EXE_ephact"));
    command.args(args).env_remove("HOME");
    command.output().expect("ephact should start")
}
