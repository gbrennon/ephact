#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShowProjectBrandingInfoResponse {
    pub name: String,
    pub description: String,
    pub version: String,
    pub emblem: String,
}
