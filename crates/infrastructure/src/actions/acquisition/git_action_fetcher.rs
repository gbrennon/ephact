use std::{
    env,
    fs::create_dir_all,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    application::ports::outbound::ActionFetcherPort,
    domain::{errors::ActionError, value_objects::RemoteActionReference},
};

const CACHE_DIRECTORY: &str = "ephact/actions";

#[derive(Clone)]
pub struct GitActionFetcher {
    cache_root: PathBuf,
}

impl GitActionFetcher {
    pub fn new(cache_root: PathBuf) -> Self {
        Self { cache_root }
    }

    pub fn with_default_cache_root() -> Self {
        let base = env::var_os("XDG_CACHE_HOME")
            .map(PathBuf::from)
            .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache")))
            .unwrap_or_else(env::temp_dir);
        Self::new(base.join(CACHE_DIRECTORY))
    }

    fn clone_shallow(url: &str, revision: &str, destination: &Path) -> Result<(), String> {
        Self::run_git(&[
            "clone".into(),
            "--depth".into(),
            "1".into(),
            "--branch".into(),
            revision.into(),
            url.into(),
            destination.display().to_string(),
        ])
    }

    fn clone_and_checkout(url: &str, revision: &str, destination: &Path) -> Result<(), String> {
        let path = destination.display().to_string();
        Self::run_git(&["clone".into(), url.into(), path.clone()])?;
        Self::run_git(&["-C".into(), path, "checkout".into(), revision.into()])
    }

    fn run_git(args: &[String]) -> Result<(), String> {
        let output = Command::new("git")
            .args(args)
            .output()
            .map_err(|error| format!("failed to run source client: {error}"))?;
        if output.status.success() {
            Ok(())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
        }
    }

    fn is_populated(directory: &Path) -> bool {
        directory
            .read_dir()
            .is_ok_and(|mut entries| entries.next().is_some())
    }

    fn source_url(reference: &RemoteActionReference) -> String {
        if reference.scheme() == "file" {
            return format!(
                "{}/{}/{}",
                reference.host(),
                reference.owner(),
                reference.repo()
            );
        }
        format!(
            "{}://{}/{}/{}",
            reference.scheme(),
            reference.host(),
            reference.owner(),
            reference.repo()
        )
    }

    fn cache_key(reference: &RemoteActionReference) -> String {
        let raw = format!(
            "{}/{}/{}/{}",
            reference.host(),
            reference.owner(),
            reference.repo(),
            reference.revision()
        );
        raw.chars()
            .map(|character| {
                if character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '_') {
                    character
                } else {
                    '_'
                }
            })
            .collect()
    }
}

impl ActionFetcherPort for GitActionFetcher {
    fn fetch(&self, reference: &RemoteActionReference) -> Result<PathBuf, ActionError> {
        let destination = self.cache_root.join(Self::cache_key(reference));
        if Self::is_populated(&destination) {
            return Ok(destination);
        }

        create_dir_all(&self.cache_root).map_err(|error| {
            ActionError::FetchFailed(format!(
                "could not create cache directory {}: {error}",
                self.cache_root.display()
            ))
        })?;

        let url = Self::source_url(reference);
        let shallow = Self::clone_shallow(&url, reference.revision(), &destination);
        if shallow.is_ok() {
            return Ok(destination);
        }

        let _ = std::fs::remove_dir_all(&destination);
        Self::clone_and_checkout(&url, reference.revision(), &destination).map_err(|error| {
            let shallow_error = shallow.unwrap_err();
            ActionError::FetchFailed(format!(
                "{url}@{}: {shallow_error}; {error}",
                reference.revision()
            ))
        })?;

        Ok(destination)
    }

    fn clone_box(&self) -> Box<dyn ActionFetcherPort> {
        Box::new(self.clone())
    }
}
