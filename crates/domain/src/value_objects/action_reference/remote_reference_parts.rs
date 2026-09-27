use crate::value_objects::RemoteActionReference;

const DEFAULT_HOST: &str = "github.com";
const DEFAULT_GIT_REF: &str = "main";

pub struct RemoteReferenceParts<'a> {
    scheme: String,
    host: String,
    owner: &'a str,
    repo: &'a str,
    git_ref: &'a str,
    directory: Vec<&'a str>,
}

impl<'a> RemoteReferenceParts<'a> {
    /// Parses a remote action reference into its component parts.
    pub fn parse(reference: &'a str) -> Option<Self> {
        let (location, git_ref) = Self::split_git_ref(reference)?;
        let (scheme, host, path) = Self::parse_scheme_host_path(location)?;
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
        let remote = RemoteActionReference::new(
            scheme,
            host,
            owner.to_string(),
            repo.to_string(),
            git_ref.to_string(),
        );
        if directory.is_empty() {
            remote
        } else {
            remote.with_directory(Some(directory.join("/")))
        }
    }

    fn split_git_ref(reference: &'a str) -> Option<(&'a str, &'a str)> {
        match reference.rsplit_once('@') {
            Some((location, git_ref)) if !location.is_empty() && !git_ref.is_empty() => {
                Some((location, git_ref))
            }
            Some(_) => None,
            None => Some((reference, DEFAULT_GIT_REF)),
        }
    }

    fn parse_scheme_host_path(location: &'a str) -> Option<(String, String, &'a str)> {
        match location.split_once("://") {
            Some((scheme, remainder)) => {
                let (host, path) = remainder.split_once('/')?;
                if scheme.is_empty() || host.is_empty() {
                    return None;
                }
                Some((scheme.to_string(), host.to_string(), path))
            }
            None => Some(("https".to_string(), DEFAULT_HOST.to_string(), location)),
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
