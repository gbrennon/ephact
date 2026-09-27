/// The defaults used to complete an under-specified remote action reference.
///
/// A shorthand `uses:` value such as `actions/checkout` omits the forge scheme,
/// host, and git ref. Deciding *which* forge a bare `owner/repo` points at is a
/// deployment policy, not a domain rule, so the domain never hardcodes it: the
/// caller (infrastructure) supplies the forge these gaps are filled from.
///
/// # Examples
///
/// ```
/// # use ephact_domain::value_objects::RemoteReferenceDefaults;
/// let defaults = RemoteReferenceDefaults::new("https".into(), "github.com".into(), "main".into());
/// assert_eq!(defaults.host(), "github.com");
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemoteReferenceDefaults {
    scheme: String,
    host: String,
    git_ref: String,
}

impl RemoteReferenceDefaults {
    /// Creates the defaults a bare reference is completed with.
    pub fn new(scheme: String, host: String, git_ref: String) -> Self {
        Self {
            scheme,
            host,
            git_ref,
        }
    }

    /// The scheme assumed when a reference omits one (e.g. `https`).
    pub fn scheme(&self) -> &str {
        &self.scheme
    }

    /// The forge host assumed when a shorthand reference omits one.
    pub fn host(&self) -> &str {
        &self.host
    }

    /// The git ref assumed when a reference omits `@ref`.
    pub fn git_ref(&self) -> &str {
        &self.git_ref
    }
}
