use crate::domain::value_objects::RemoteActionReference;

/// Request DTO for the
/// [`FetchRemoteActionPort`](crate::ports::inbound::fetch_remote_action_port::FetchRemoteActionPort)
/// inbound port.
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
