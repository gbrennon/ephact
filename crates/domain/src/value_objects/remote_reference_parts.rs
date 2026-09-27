use crate::value_objects::{RemoteActionReference, RemoteReferenceDefaults};

pub struct RemoteReferenceParts<'a> {
    scheme: String,
    host: String,
    owner: &'a str,
    repo: &'a str,
    git_ref: String,
    directory: Vec<&'a str>,
}

impl<'a> RemoteReferenceParts<'a> {
    /// Parses a remote action reference into its component parts, filling any
    /// omitted scheme, host, or git ref from `defaults`.
    pub fn parse(reference: &'a str, defaults: &RemoteReferenceDefaults) -> Option<Self> {
        let (location, git_ref) = Self::split_git_ref(reference, defaults)?;
        let (scheme, host, path) = Self::parse_scheme_host_path(location, defaults)?;
        let (owner, repo, directory) = Self::parse_segments(path)?;

        Some(Self {
            scheme,
            host,
            owner,
            repo,
            git_ref,
            directory,
        })
    }

    /// Converts parsed reference parts into a remote action reference.
    pub fn into_remote(self) -> RemoteActionReference {
        let Self {
            scheme,
            host,
            owner,
            repo,
            git_ref,
            directory,
        } = self;
        let directory = (!directory.is_empty()).then(|| directory.join("/"));
        RemoteActionReference::new(scheme, host, owner.to_string(), repo.to_string(), git_ref)
            .with_directory(directory)
    }

    fn split_git_ref(
        reference: &'a str,
        defaults: &RemoteReferenceDefaults,
    ) -> Option<(&'a str, String)> {
        match reference.rsplit_once('@') {
            Some((location, git_ref)) if !location.is_empty() && !git_ref.is_empty() => {
                Some((location, git_ref.to_string()))
            }
            Some(_) => None,
            None => Some((reference, defaults.git_ref().to_string())),
        }
    }

    fn parse_scheme_host_path(
        location: &'a str,
        defaults: &RemoteReferenceDefaults,
    ) -> Option<(String, String, &'a str)> {
        match location.split_once("://") {
            Some((scheme, remainder)) => {
                let (host, path) = remainder.split_once('/')?;
                (!scheme.is_empty() && !host.is_empty())
                    .then_some((scheme.to_string(), host.to_string(), path))
            }
            None => Some((
                defaults.scheme().to_string(),
                defaults.host().to_string(),
                location,
            )),
        }
    }

    fn parse_segments(path: &'a str) -> Option<(&'a str, &'a str, Vec<&'a str>)> {
        let mut segments = path.split('/').filter(|segment| !segment.is_empty());
        let owner = segments.next()?;
        let repo = segments.next()?;
        let directory = segments.collect();

        Some((owner, repo, directory))
    }
}
