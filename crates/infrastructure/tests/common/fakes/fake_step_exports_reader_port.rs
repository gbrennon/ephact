use std::{collections::HashMap, sync::Arc};

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
    queued: Arc<Mutex<Vec<QueuedStepExports>>>,
    output_queued: Arc<Mutex<Vec<HashMap<String, String>>>>,
    calls: Arc<Mutex<usize>>,
}

impl FakeStepExportsReaderPort {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn queueing(exports: Vec<QueuedStepExports>) -> Self {
        Self {
            queued: Arc::new(Mutex::new(exports)),
            output_queued: Arc::new(Mutex::new(Vec::new())),
            calls: Arc::new(Mutex::new(0)),
        }
    }

    pub fn queueing_with_outputs(exports: Vec<QueuedStepExportsWithOutputs>) -> Self {
        let mut queued = Vec::with_capacity(exports.len());
        let mut output_queued = Vec::with_capacity(exports.len());
        for (path_additions, env, outputs) in exports {
            queued.push((path_additions, env));
            output_queued.push(outputs);
        }
        Self {
            queued: Arc::new(Mutex::new(queued)),
            output_queued: Arc::new(Mutex::new(output_queued)),
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
        let mut queued = self.queued.lock();
        if queued.is_empty() {
            return StepExportsResponse::new(Vec::new(), HashMap::new());
        }
        let (path_additions, env) = queued.remove(0);
        let outputs = {
            let mut output_queued = self.output_queued.lock();
            if output_queued.is_empty() {
                HashMap::new()
            } else {
                output_queued.remove(0)
            }
        };
        StepExportsResponse::new(path_additions, env).with_outputs(outputs)
    }
}
