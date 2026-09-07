use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::mem::ManuallyDrop;
use std::os::unix::io::FromRawFd;
use std::thread::JoinHandle;

const SUPPRESSED_LINE_PREFIXES: [&str; 2] = [
    "Error: crun: executable file `bash` not found",
    "Error: read unixpacket",
];

/// Duplicated stderr whose lines matching [`SUPPRESSED_LINE_PREFIXES`] are
/// dropped before reaching the terminal.
pub struct StderrFilter {
    filter_thread: Option<JoinHandle<()>>,
    restored_fd: libc::c_int,
}

impl StderrFilter {
    /// Installs a pipe in front of the process stderr and spawns the draining
    /// thread. Falls back to no filtering when stderr is not duplicable.
    pub fn install() -> Self {
        let restored_fd = unsafe { libc::dup(libc::STDERR_FILENO) };
        if restored_fd < 0 {
            return Self {
                filter_thread: None,
                restored_fd: -1,
            };
        }

        let mut fds: [libc::c_int; 2] = [0; 2];
        if unsafe { libc::pipe(fds.as_mut_ptr()) } != 0 {
            return Self {
                filter_thread: None,
                restored_fd,
            };
        }

        let filter_thread = Some(spawn_filter_thread(fds[0], restored_fd));
        unsafe { libc::dup2(fds[1], libc::STDERR_FILENO) };
        unsafe { libc::close(fds[1]) };

        Self {
            filter_thread,
            restored_fd,
        }
    }

    /// Restores the original stderr and waits for the draining thread.
    pub fn restore(mut self) {
        if self.restored_fd >= 0 {
            unsafe { libc::dup2(self.restored_fd, libc::STDERR_FILENO) };
            unsafe { libc::close(self.restored_fd) };
        }
        if let Some(handle) = self.filter_thread.take() {
            let _ = handle.join();
        }
    }
}

fn spawn_filter_thread(read_end: libc::c_int, real_stderr: libc::c_int) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let reader = BufReader::new(unsafe { File::from_raw_fd(read_end) });
        let mut real = ManuallyDrop::new(unsafe { File::from_raw_fd(real_stderr) });
        for line in reader.lines().map_while(Result::ok) {
            if line_should_be_suppressed(&line) {
                continue;
            }
            let _ = writeln!(&*real, "{line}");
            let _ = real.flush();
        }
    })
}

fn line_should_be_suppressed(line: &str) -> bool {
    SUPPRESSED_LINE_PREFIXES
        .iter()
        .any(|prefix| line.starts_with(prefix))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crun_missing_bash_line_is_suppressed() {
        assert!(line_should_be_suppressed(
            "Error: crun: executable file `bash` not found in $PATH"
        ));
    }

    #[test]
    fn unix_packet_error_line_is_suppressed() {
        assert!(line_should_be_suppressed(
            "Error: read unixpacket @/run/whatever: broken pipe"
        ));
    }

    #[test]
    fn ordinary_lines_are_not_suppressed() {
        assert!(!line_should_be_suppressed("Compiling ephact v0.1.0"));
        assert!(!line_should_be_suppressed(""));
        assert!(!line_should_be_suppressed(
            "Warning: crun: executable file `bash` not found"
        ));
    }

    #[test]
    fn filter_thread_never_closes_the_real_stderr_descriptor() {
        let mut pipe_fds: [libc::c_int; 2] = [0; 2];
        assert_eq!(unsafe { libc::pipe(pipe_fds.as_mut_ptr()) }, 0);
        let real_stderr = unsafe { libc::dup(libc::STDERR_FILENO) };
        assert!(real_stderr >= 0);

        let handle = spawn_filter_thread(pipe_fds[0], real_stderr);
        unsafe { libc::close(pipe_fds[1]) };
        handle.join().expect("filter thread must not panic");

        assert!(
            unsafe { libc::fcntl(real_stderr, libc::F_GETFD) } != -1,
            "the real stderr descriptor must remain open after the thread exits"
        );
        unsafe { libc::close(real_stderr) };
    }

    #[test]
    fn install_and_restore_leave_stderr_usable() {
        let filter = StderrFilter::install();
        eprintln!("message written through the installed filter");
        filter.restore();
        eprintln!("message written after restore");
    }
}
