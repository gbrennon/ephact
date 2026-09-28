mod app_runner;

use std::error::Error;

use ephact::infrastructure::logging::stderr_filter::StderrFilter;

trait ExitStrategy {
    fn exit(&self, code: i32);
}

struct ProcessExit;

impl ExitStrategy for ProcessExit {
    fn exit(&self, code: i32) {
        std::process::exit(code);
    }
}

fn main() {
    run_application(&ProcessExit);
}

fn run_application<E: ExitStrategy>(exit_strategy: &E) {
    let stderr_filter = StderrFilter::install();
    let result = app_runner::run_application();
    stderr_filter.restore();
    finish(result, exit_strategy);
}

fn finish<E: ExitStrategy>(result: Result<(), Box<dyn Error>>, exit_strategy: &E) {
    match result {
        Ok(()) => exit_strategy.exit(0),
        Err(error) => {
            eprintln!("Error: {error}");
            exit_strategy.exit(1);
        }
    }
}

#[cfg(test)]
mod main_tests;
