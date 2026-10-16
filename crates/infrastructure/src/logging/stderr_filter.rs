use std::{
    fs::File,
    io::{self, BufRead, BufReader, Write},
    os::{fd::AsFd, unix::io::OwnedFd},
    thread::JoinHandle,
};

use rustix::{io::dup, pipe::pipe, stdio::dup2_stderr};

const SUPPRESSED_LINE_PREFIXES: [&str; 2] = [
    "Error: crun: executable file `bash` not found",
    "Error: read unixpacket",
];

/// Duplicated stderr whose lines matching [`SUPPRESSED_LINE_PREFIXES`] are
/// dropped before reaching the terminal.
pub struct StderrFilter {
    filter_thread: Option<JoinHandle<()>>,
    restored_fd: Option<OwnedFd>,
}

impl StderrFilter {
    /// Installs a pipe in front of the process stderr and spawns the draining
    /// thread. Falls back to no filtering when stderr is not duplicable.
    pub fn install() -> Self {
        let stderr = io::stderr();
        let restored_fd = match dup(stderr.as_fd()) {
            Ok(fd) => fd,
            Err(_) => return Self::disabled(),
        };
        let (read_end, write_end) = match pipe() {
            Ok(fds) => fds,
            Err(_) => return Self::disabled(),
        };
        let thread_stderr = match dup(&restored_fd) {
            Ok(fd) => fd,
            Err(_) => return Self::disabled(),
        };
        if dup2_stderr(&write_end).is_err() {
            return Self::disabled();
        }
        let filter_thread = Some(spawn_filter_thread(read_end, thread_stderr));
        drop(write_end);

        Self {
            filter_thread,
            restored_fd: Some(restored_fd),
        }
    }

    /// Restores the original stderr and waits for the draining thread.
    pub fn restore(mut self) {
        if let Some(restored_fd) = self.restored_fd.as_ref() {
            let _ = dup2_stderr(restored_fd);
        }
        if let Some(handle) = self.filter_thread.take() {
            let _ = handle.join();
        }
    }

    fn disabled() -> Self {
        Self {
            filter_thread: None,
            restored_fd: None,
        }
    }

    fn line_should_be_suppressed(line: &str) -> bool {
        SUPPRESSED_LINE_PREFIXES
            .iter()
            .any(|prefix| line.starts_with(prefix))
    }
}

fn spawn_filter_thread(read_end: OwnedFd, real_stderr: OwnedFd) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let reader = BufReader::new(File::from(read_end));
        let mut real = File::from(real_stderr);
        for line in reader.lines().map_while(Result::ok) {
            if StderrFilter::line_should_be_suppressed(&line) {
                continue;
            }
            let _ = writeln!(real, "{line}");
            let _ = real.flush();
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crun_missing_bash_line_is_suppressed() {
        assert!(StderrFilter::line_should_be_suppressed(
            "Error: crun: executable file `bash` not found in $PATH"
        ));
    }

    #[test]
    fn unix_packet_error_line_is_suppressed() {
        assert!(StderrFilter::line_should_be_suppressed(
            "Error: read unixpacket @/run/whatever: broken pipe"
        ));
    }

    #[test]
    fn ordinary_lines_are_not_suppressed() {
        assert!(!StderrFilter::line_should_be_suppressed(
            "Compiling ephact v0.1.0"
        ));
        assert!(!StderrFilter::line_should_be_suppressed(""));
        assert!(!StderrFilter::line_should_be_suppressed(
            "Warning: crun: executable file `bash` not found"
        ));
    }

    #[test]
    fn filter_thread_does_not_close_the_original_stderr_descriptor() {
        let stderr = io::stderr();
        let original_stderr = dup(stderr.as_fd()).expect("stderr must be duplicable");
        let thread_stderr = dup(&original_stderr).expect("stderr must be duplicable");
        let (read_end, write_end) = pipe().expect("pipe must be creatable");

        let handle = spawn_filter_thread(read_end, thread_stderr);
        drop(write_end);
        handle.join().expect("filter thread must not panic");

        assert!(dup(&original_stderr).is_ok());
    }

    #[test]
    fn install_and_restore_leave_stderr_usable() {
        let filter = StderrFilter::install();
        eprintln!("message written through the installed filter");
        filter.restore();
        eprintln!("message written after restore");
    }
}
