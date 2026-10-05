use tokio::runtime::Handle;

/// Runs a future to completion on a supplied Tokio runtime handle, including
/// when the caller is already inside a Tokio runtime.
pub struct RuntimeBlocker {
    runtime: Handle,
}

impl RuntimeBlocker {
    /// Stores the runtime handle used by later calls to [`Self::block_on`].
    pub fn new(runtime: &Handle) -> Self {
        Self {
            runtime: runtime.clone(),
        }
    }

    /// Runs `future` synchronously on the stored runtime and returns its output.
    pub fn block_on<F: Future>(&self, future: F) -> F::Output {
        if Handle::try_current().is_ok() {
            tokio::task::block_in_place(|| self.runtime.block_on(future))
        } else {
            self.runtime.block_on(future)
        }
    }
}

#[cfg(test)]
mod tests {
    use tokio::runtime::Runtime;

    use super::RuntimeBlocker;

    #[tokio::test(flavor = "multi_thread")]
    async fn block_on_runs_a_future_from_within_a_runtime() {
        let result = tokio::task::block_in_place(|| {
            let runtime = Runtime::new().expect("runtime");
            RuntimeBlocker::new(runtime.handle()).block_on(async { 7 })
        });

        assert_eq!(result, 7);
    }
}
