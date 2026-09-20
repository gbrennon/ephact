use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

use crate::{
    domain::messages::events::{
        DomainEvent, JobStartedPayload, StepFinishedPayload, StepOutputPayload,
    },
    infrastructure::messaging::DomainEventHandler,
};
/// Presentation handler that renders workflow run progress to the terminal.
///
/// In non-verbose mode, steps are announced as they begin and their outcome is
/// reported on completion, while workflow and job headers, live step output,
/// and failure diagnostics remain hidden.
///
/// In verbose mode, workflow and job lifecycle events are announced, step output
/// is relayed in real time, and failure diagnostics are appended on step completion.
///
/// Everything is written to standard error so that step output relayed to
/// standard output stays clean.
pub struct RunProgressHandler {
    verbose: bool,
    tui_sender: Option<UnboundedSender<String>>,
    output_suppressed: Arc<AtomicBool>,
}

pub struct TuiProgressStream {
    receiver: Arc<Mutex<UnboundedReceiver<String>>>,
    output_suppressed: Arc<AtomicBool>,
}

impl TuiProgressStream {
    pub fn try_recv(&self) -> Option<String> {
        self.receiver
            .lock()
            .expect("progress receiver lock")
            .try_recv()
            .ok()
    }

    pub fn activate_tui(&self) {
        self.output_suppressed.store(true, Ordering::Relaxed);
    }

    pub fn deactivate_tui(&self) {
        self.output_suppressed.store(false, Ordering::Relaxed);
    }
}

impl RunProgressHandler {
    pub fn new(verbose: bool) -> Self {
        Self {
            verbose,
            tui_sender: None,
            output_suppressed: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn with_tui_stream(verbose: bool) -> (Self, TuiProgressStream) {
        let (sender, receiver) = tokio::sync::mpsc::unbounded_channel();
        let output_suppressed = Arc::new(AtomicBool::new(false));
        let stream = TuiProgressStream {
            receiver: Arc::new(Mutex::new(receiver)),
            output_suppressed: output_suppressed.clone(),
        };
        let handler = Self {
            verbose,
            tui_sender: Some(sender),
            output_suppressed,
        };
        (handler, stream)
    }

    fn write_line(line: &str) {
        let mut stderr = std::io::stderr().lock();
        let _ = writeln!(stderr, "{line}");
        let _ = stderr.flush();
    }

    fn status(success: bool) -> &'static str {
        if success { "succeeded" } else { "failed" }
    }

    fn job_label(payload: &JobStartedPayload) -> String {
        match payload.job_name() {
            Some(name) => format!("{} ({})", payload.job_id(), name),
            None => payload.job_id().to_string(),
        }
    }

    fn output_line(payload: &StepOutputPayload) -> String {
        format!("      | {}", payload.text())
    }

    fn relay_output(payload: &StepOutputPayload) {
        let mut stderr = std::io::stderr().lock();
        let _ = stderr.write_all(Self::output_line(payload).as_bytes());
        let _ = stderr.flush();
    }

    fn step_status(exit_code: Option<i64>) -> String {
        match exit_code {
            Some(0) => "ok".to_string(),
            Some(code) => format!("failed (exit code: {code})"),
            None => "error".to_string(),
        }
    }

    fn step_outcome(&self, payload: &StepFinishedPayload) -> String {
        let outcome = Self::step_status(payload.exit_code());
        let mut output = format!("    Step '{}': {outcome}", payload.step_name());
        if self.verbose && payload.exit_code() != Some(0) {
            Self::append_failure_output(&mut output, "stdout", payload.stdout());
            Self::append_failure_output(&mut output, "stderr", payload.stderr());
        }
        output
    }

    fn append_failure_output(output: &mut String, label: &str, text: &str) {
        if text.is_empty() {
            return;
        }
        for line in text.lines() {
            output.push_str(&format!("\n      {label}: {line}"));
        }
    }

    /// Renders the terminal line for an event, or `None` when the event is
    /// not shown in the current verbosity mode.
    fn render(&self, event: &DomainEvent) -> Option<String> {
        match event {
            DomainEvent::WorkflowStarted(payload) if self.verbose => {
                Some(format!("Workflow '{}'", payload.workflow_name()))
            }
            DomainEvent::JobStarted(payload) if self.verbose => {
                Some(format!("  Job '{}'", Self::job_label(payload)))
            }
            DomainEvent::JobFinished(payload) if self.verbose => Some(format!(
                "  Job '{}': {}",
                payload.job_id(),
                Self::status(payload.success())
            )),
            DomainEvent::StepStarted(payload) => {
                Some(format!("    Step '{}': running...", payload.step_name()))
            }
            DomainEvent::StepFinished(payload) => Some(self.step_outcome(payload)),
            _ => None,
        }
    }

    fn renders_output(&self) -> bool {
        self.verbose
    }
}

impl DomainEventHandler for RunProgressHandler {
    fn handle(&self, event: &DomainEvent) {
        self.stream_progress(event);
        if self.output_suppressed.load(Ordering::Relaxed) {
            return;
        }
        match event {
            DomainEvent::StepOutput(payload) if self.renders_output() => {
                Self::relay_output(payload)
            }
            other => {
                if let Some(line) = self.render(other) {
                    Self::write_line(&line);
                }
            }
        }
    }
}

impl RunProgressHandler {
    fn stream_progress(&self, event: &DomainEvent) {
        if !self.output_suppressed.load(Ordering::Relaxed) {
            return;
        }
        let Some(sender) = &self.tui_sender else {
            return;
        };
        if let Some(line) = self.render(event) {
            let _ = sender.send(line);
        } else if self.renders_output()
            && let DomainEvent::StepOutput(payload) = event
        {
            let _ = sender.send(Self::output_line(payload));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::messages::events::{
        JobFinishedPayload, JobStartedPayload, OutputStream, StepFinishedDetails,
        StepFinishedPayload, StepOutputPayload, StepStartedPayload, WorkflowStartedPayload,
    };

    fn step_started() -> DomainEvent {
        DomainEvent::StepStarted(StepStartedPayload::new(
            "Build".into(),
            "build".into(),
            "compile".into(),
        ))
    }

    fn step_output(stream: OutputStream) -> DomainEvent {
        DomainEvent::StepOutput(StepOutputPayload::new(
            "compile".into(),
            stream,
            "Compiling ephact\n".into(),
        ))
    }

    fn step_finished(exit_code: Option<i64>) -> DomainEvent {
        DomainEvent::StepFinished(StepFinishedPayload::new(
            "run-1".into(),
            StepFinishedDetails::new(
                "Build".into(),
                "build".into(),
                "compile".into(),
                exit_code == Some(0),
                exit_code,
            )
            .with_stdout(String::new())
            .with_stderr(String::new()),
        ))
    }

    #[test]
    fn quiet_mode_announces_running_steps() {
        let handler = RunProgressHandler::new(false);
        assert_eq!(
            handler.render(&step_started()).as_deref(),
            Some("    Step 'compile': running...")
        );
    }

    #[test]
    fn tui_stream_receives_rendered_progress_lines() {
        let (handler, stream) = RunProgressHandler::with_tui_stream(false);
        stream.activate_tui();

        handler.handle(&step_started());

        assert_eq!(
            stream.try_recv().as_deref(),
            Some("    Step 'compile': running...")
        );
    }

    #[test]
    fn inactive_tui_stream_does_not_buffer_cli_progress() {
        let (handler, stream) = RunProgressHandler::with_tui_stream(false);

        handler.handle(&DomainEvent::WorkflowStarted(WorkflowStartedPayload::new(
            "Build".into(),
        )));

        assert!(stream.try_recv().is_none());
    }

    #[test]
    fn quiet_mode_reports_step_status() {
        let handler = RunProgressHandler::new(false);
        assert_eq!(
            handler.render(&step_finished(Some(0))).as_deref(),
            Some("    Step 'compile': ok")
        );
        assert!(!handler.renders_output());
    }

    #[test]
    fn quiet_mode_hides_workflow_and_job_headers() {
        let handler = RunProgressHandler::new(false);
        assert!(
            handler
                .render(&DomainEvent::WorkflowStarted(WorkflowStartedPayload::new(
                    "Build".into(),
                )))
                .is_none()
        );
        assert!(
            handler
                .render(&DomainEvent::JobStarted(JobStartedPayload::new(
                    "Build".into(),
                    "build".into(),
                    Some("Build".into()),
                )))
                .is_none()
        );
        assert!(
            handler
                .render(&DomainEvent::JobFinished(JobFinishedPayload::new(
                    "Build".into(),
                    "build".into(),
                    Some("Build".into()),
                    true,
                )))
                .is_none()
        );
    }

    #[test]
    fn quiet_mode_hides_failed_step_output() {
        let handler = RunProgressHandler::new(false);
        let event = DomainEvent::StepFinished(StepFinishedPayload::new(
            "run-1".into(),
            StepFinishedDetails::new(
                "Build".into(),
                "build".into(),
                "clippy".into(),
                false,
                Some(101),
            )
            .with_stdout("stdout text".into())
            .with_stderr("clippy failed".into()),
        ));

        let rendered = handler.render(&event);
        assert_eq!(
            rendered.as_deref(),
            Some("    Step 'clippy': failed (exit code: 101)")
        );
        let rendered_text = rendered.unwrap();
        assert!(!rendered_text.contains("stdout"));
        assert!(!rendered_text.contains("clippy failed"));
    }

    #[test]
    fn verbose_mode_reports_failed_step_output() {
        let handler = RunProgressHandler::new(true);
        let event = DomainEvent::StepFinished(StepFinishedPayload::new(
            "run-1".into(),
            StepFinishedDetails::new(
                "Build".into(),
                "build".into(),
                "clippy".into(),
                false,
                Some(101),
            )
            .with_stdout("stdout text".into())
            .with_stderr("clippy failed".into()),
        ));

        let rendered = handler.render(&event).unwrap();
        assert!(rendered.contains("Step 'clippy': failed (exit code: 101)"));
        assert!(rendered.contains("stderr: clippy failed"));
    }
    #[test]
    fn verbose_mode_announces_running_steps() {
        let handler = RunProgressHandler::new(true);
        assert_eq!(
            handler.render(&step_started()).as_deref(),
            Some("    Step 'compile': running...")
        );
        assert!(handler.renders_output());
    }

    #[test]
    fn verbose_mode_relays_step_output_and_quiet_mode_does_not() {
        let verbose = RunProgressHandler::new(true);
        let quiet = RunProgressHandler::new(false);
        let event = step_output(OutputStream::StandardOutput);
        assert!(verbose.render(&event).is_none() && verbose.renders_output());
        assert!(!quiet.renders_output());
    }

    #[test]
    fn verbose_mode_still_reports_the_step_status() {
        let handler = RunProgressHandler::new(true);
        assert_eq!(
            handler.render(&step_finished(Some(0))).as_deref(),
            Some("    Step 'compile': ok")
        );
    }
}
