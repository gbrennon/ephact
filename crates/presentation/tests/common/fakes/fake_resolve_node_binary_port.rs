use ephact::application::{
    dtos::requests::ResolveNodeBinaryRequest, ports::outbound::ResolveNodeBinaryPort,
};

/// Reports a prepared node interpreter.
#[derive(Clone)]
pub struct FakeResolveNodeBinaryPort {
    binary: String,
}

impl FakeResolveNodeBinaryPort {
    pub fn returning(binary: &str) -> Self {
        Self {
            binary: binary.to_string(),
        }
    }
}

impl ResolveNodeBinaryPort for FakeResolveNodeBinaryPort {
    fn resolve(&self, _request: ResolveNodeBinaryRequest) -> String {
        self.binary.clone()
    }
}
