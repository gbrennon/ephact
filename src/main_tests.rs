use std::cell::RefCell;

use super::{ExitStrategy, finish};

struct RecordingExit(RefCell<Vec<i32>>);

impl RecordingExit {
    fn new() -> Self {
        Self(RefCell::new(Vec::new()))
    }

    fn codes(&self) -> Vec<i32> {
        self.0.borrow().clone()
    }
}

impl ExitStrategy for RecordingExit {
    fn exit(&self, code: i32) {
        self.0.borrow_mut().push(code);
    }
}

#[test]
fn custom_exit_strategy_receives_success_code() {
    let exit = RecordingExit::new();

    finish(Ok(()), &exit);

    assert_eq!(exit.codes(), vec![0]);
}

#[test]
fn custom_exit_strategy_receives_failure_code() {
    let exit = RecordingExit::new();

    finish(Err("failure".into()), &exit);

    assert_eq!(exit.codes(), vec![1]);
}
