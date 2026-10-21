use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

use ephact::application::{
    dtos::{requests::ReadStepExportsRequest, responses::StepExportsResponse},
    ports::outbound::step_exports_reader_port::StepExportsReaderPort,
};
use parking_lot::Mutex;

type QueuedStepExports = (Vec<String>, HashMap<String, String>);

/// Hands out the next queued set of exports, or nothing once drained.
#[derive(Clone, Default)]
pub struct FakeStepExportsReaderPort {
    queued: Arc<Mutex<VecDeque<QueuedStepExports>>>,
    calls: Arc<Mutex<usize>>,
}

impl FakeStepExportsReaderPort {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn queueing(exports: Vec<QueuedStepExports>) -> Self {
        Self {
            queued: Arc::new(Mutex::new(VecDeque::from(exports))),
            calls: Arc::new(Mutex::new(0)),
        }
    }

    pub fn calls(&self) -> usize {
        *self.calls.lock()
    }
}

impl StepExportsReaderPort for FakeStepExportsReaderPort {
    fn read(
        &self,
        _request: ReadStepExportsRequest,
        _container: &dyn ephact::application::ports::outbound::ContainerPort,
    ) -> StepExportsResponse {
        *self.calls.lock() += 1;
        let queued = self.queued.lock();
        if queued.is_empty() {
            return StepExportsResponse::new(Vec::new(), HashMap::new());
        }
        let (path_additions, env) = queued.front().cloned().expect("checked non-empty");
        StepExportsResponse::new(path_additions, env)
    }
}
