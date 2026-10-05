use crate::domain::value_objects::RemoteActionReference;

/// Request data for the outbound operation.
/// The request identifies the remote action reference to fetch.
pub struct FetchRemoteActionRequest {
    reference: RemoteActionReference,
}

impl FetchRemoteActionRequest {
    pub fn new(reference: RemoteActionReference) -> Self {
        Self { reference }
    }

    pub fn reference(&self) -> &RemoteActionReference {
        &self.reference
    }
}
