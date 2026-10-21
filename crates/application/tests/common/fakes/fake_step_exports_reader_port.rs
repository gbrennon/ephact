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
type QueuedStepExportsWithOutputs = (
    Vec<String>,
    HashMap<String, String>,
    HashMap<String, String>,
);

/// Hands out the next queued set of exports, or nothing once drained.
#[derive(Clone, Default)]
pub struct FakeStepExportsReaderPort {
    queued: Arc<Mutex<VecDeque<QueuedStepExports>>>,
    output_queued: Arc<Mutex<VecDeque<HashMap<String, String>>>>,
    calls: Arc<Mutex<usize>>,
}

impl FakeStepExportsReaderPort {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn queueing(exports: Vec<QueuedStepExports>) -> Self {
        Self {
            queued: Arc::new(Mutex::new(exports.into())),
            output_queued: Arc::new(Mutex::new(VecDeque::new())),
            calls: Arc::new(Mutex::new(0)),
        }
    }
    pub fn queueing_with_outputs(exports: Vec<QueuedStepExportsWithOutputs>) -> Self {
        let (queued, output_queued): (Vec<QueuedStepExports>, Vec<HashMap<String, String>>) =
            exports
                .into_iter()
                .map(|(path_additions, env, outputs)| ((path_additions, env), outputs))
                .unzip();
        Self {
            queued: Arc::new(Mutex::new(queued.into())),
            output_queued: Arc::new(Mutex::new(output_queued.into())),
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
        let Some((path_additions, env)) = self.queued.lock().pop_front() else {
            return StepExportsResponse::new(Vec::new(), HashMap::new());
        };
        let outputs = self.output_queued.lock().pop_front().unwrap_or_default();
        StepExportsResponse::new(path_additions, env).with_outputs(outputs)
    }
}
