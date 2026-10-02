use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use ephact::application::{
    dtos::requests::ReadStepOutputExportsRequest, ports::outbound::ReadStepOutputExportsPort,
};

/// Returns prepared step outputs, recording that it was consulted.
#[derive(Clone)]
pub struct FakeReadStepOutputExportsPort {
    outputs: HashMap<String, String>,
    called: Arc<AtomicBool>,
}

impl FakeReadStepOutputExportsPort {
    pub fn returning(outputs: HashMap<String, String>) -> Self {
        Self {
            outputs,
            called: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn was_called(&self) -> bool {
        self.called.load(Ordering::SeqCst)
    }
}

impl ReadStepOutputExportsPort for FakeReadStepOutputExportsPort {
    fn read(
        &self,
        _request: ReadStepOutputExportsRequest,
        _container: &dyn ephact::application::ports::outbound::ContainerPort,
    ) -> HashMap<String, String> {
        self.called.store(true, Ordering::SeqCst);
        self.outputs.clone()
    }
}
