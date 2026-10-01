use crate::value_objects::{RemoteActionReference, RemoteReferenceDefaults};

pub struct RemoteReferenceParts<'a> {
    scheme: String,
    host: String,
    owner: &'a str,
    repo: &'a str,
    directory: Option<String>,
    revision: String,
}

impl<'a> RemoteReferenceParts<'a> {
    pub fn parse(reference: &'a str, defaults: &RemoteReferenceDefaults) -> Option<Self> {
        let (location, revision) = Self::split_revision(reference, defaults)?;
        let (scheme, host, path) = Self::split_location(location, defaults);
        let (owner, repo, directory) = Self::split_repository_path(path)?;
        Some(Self {
            scheme,
            host,
            owner,
            repo,
            directory,
            revision,
        })
    }

    pub fn into_remote(self) -> RemoteActionReference {
        RemoteActionReference::new(
            self.scheme,
            self.host,
            self.owner.to_owned(),
            self.repo.to_owned(),
            self.revision,
        )
        .with_directory(self.directory)
    }

    fn split_revision(
        reference: &'a str,
        defaults: &RemoteReferenceDefaults,
    ) -> Option<(&'a str, String)> {
        match reference.split_once('@') {
            Some((location, revision)) if !location.is_empty() && !revision.is_empty() => {
                Some((location, revision.to_owned()))
            }
            None => Some((reference, defaults.revision().to_owned())),
            _ => None,
        }
    }
    fn split_location<'b>(
        location: &'b str,
        defaults: &RemoteReferenceDefaults,
    ) -> (String, String, &'b str) {
        if let Some((scheme, rest)) = location.split_once("://") {
            let (host, path) = rest.split_once('/').unwrap_or((rest, ""));
            (scheme.to_owned(), host.to_owned(), path)
        } else {
            (
                defaults.scheme().to_owned(),
                defaults.host().to_owned(),
                location,
            )
        }
    }

    fn split_repository_path(path: &'a str) -> Option<(&'a str, &'a str, Option<String>)> {
        let mut segments = path.split('/').filter(|segment| !segment.is_empty());
        let owner = segments.next()?;
        let repo = segments.next()?;
        if repo.is_empty() {
            return None;
        }
        let directory = segments.collect::<Vec<_>>();
        Some((
            owner,
            repo,
            (!directory.is_empty()).then(|| directory.join("/")),
        ))
    }
}
