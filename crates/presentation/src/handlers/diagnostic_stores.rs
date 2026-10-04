use crate::infrastructure::logging::{FailureLogErrorStore, FailureLogPathStore};

pub struct DiagnosticStores<'a> {
    error_store: &'a FailureLogErrorStore,
    path_store: &'a FailureLogPathStore,
}

impl<'a> DiagnosticStores<'a> {
    pub fn new(error_store: &'a FailureLogErrorStore, path_store: &'a FailureLogPathStore) -> Self {
        Self {
            error_store,
            path_store,
        }
    }

    pub(super) fn error_store(&self) -> &FailureLogErrorStore {
        self.error_store
    }

    pub(super) fn path_store(&self) -> &FailureLogPathStore {
        self.path_store
    }
}
