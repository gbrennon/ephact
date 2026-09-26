use std::{cell::Cell, collections::HashSet};

use ratatui::{
    Frame,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{List, ListItem, ListState},
};

use crate::{
    application::dtos::responses::{RunSummaryResponse, StepSummaryResponse},
    tui::theme::Theme,
};

const SUCCESS_LABEL: &str = "SUCCESS";
const FAILURE_LABEL: &str = "FAILED";
const EXPANDED_MARK: &str = "v ";
const COLLAPSED_MARK: &str = "> ";
const LEAF_MARK: &str = "  ";
const DEFAULT_WIDTH: u16 = 80;

#[derive(Clone, Copy)]
enum Node {
    Workflow,
    Job(usize),
    Step(usize, usize),
}

enum Row {
    Node(Node, Line<'static>),
    Output(Line<'static>),
}

#[derive(Clone)]
pub struct RunDetailsView {
    workflow_expanded: bool,
    collapsed_jobs: HashSet<usize>,
    collapsed_outputs: HashSet<(usize, usize)>,
    cursor: usize,
    last_row_count: Cell<usize>,
    last_width: Cell<u16>,
}

impl RunDetailsView {
    pub fn new() -> Self {
        Self {
            workflow_expanded: true,
            collapsed_jobs: HashSet::new(),
            collapsed_outputs: HashSet::new(),
            cursor: 0,
            last_row_count: Cell::new(1),
            last_width: Cell::new(DEFAULT_WIDTH),
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new();
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn move_up(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    pub fn move_down(&mut self, summary: &RunSummaryResponse) {
        self.refresh_row_count(summary);
        let last = self.last_row_count.get().saturating_sub(1);
        self.cursor = self.cursor.saturating_add(1).min(last);
    }

    pub fn toggle(&mut self, summary: &RunSummaryResponse) {
        self.refresh_row_count(summary);
        let rows = self.rows(summary, self.last_width.get());
        let Some(node) = rows.get(self.cursor).and_then(row_node) else {
            return;
        };
        match node {
            Node::Workflow => self.workflow_expanded = !self.workflow_expanded,
            Node::Job(job) => toggle_membership(&mut self.collapsed_jobs, job),
            Node::Step(job, step) => {
                toggle_membership(&mut self.collapsed_outputs, (job, step));
            }
        }
        self.refresh_row_count(summary);
        self.cursor = self.cursor.min(self.last_row_count.get().saturating_sub(1));
    }

    pub fn render(&self, frame: &mut Frame<'_>, area: Rect, summary: &RunSummaryResponse) {
        self.last_width.set(area.width.max(1));
        let rows = self.rows(summary, self.last_width.get());
        self.last_row_count.set(rows.len());
        let selected = self.cursor.min(self.last_row_count.get().saturating_sub(1));
        let items: Vec<ListItem<'static>> = rows.into_iter().map(row_item).collect();
        let list = List::new(items)
            .style(Theme::body_style())
            .highlight_style(Theme::selection_style());
        let mut state = ListState::default();
        state.select(Some(selected));
        *state.offset_mut() = list_offset(selected, self.last_row_count.get(), area.height);
        frame.render_stateful_widget(list, area, &mut state);
    }

    fn refresh_row_count(&self, summary: &RunSummaryResponse) {
        self.last_row_count
            .set(self.rows(summary, self.last_width.get()).len());
    }

    fn rows(&self, summary: &RunSummaryResponse, width: u16) -> Vec<Row> {
        let mut rows = vec![Row::Node(Node::Workflow, self.workflow_line(summary))];
        if !self.workflow_expanded {
            return rows;
        }
        for (job, _) in summary.job_summaries().iter().enumerate() {
            self.append_job_rows(&mut rows, job, summary, width);
        }
        rows
    }

    fn append_job_rows(
        &self,
        rows: &mut Vec<Row>,
        job: usize,
        summary: &RunSummaryResponse,
        width: u16,
    ) {
        let summary_job = &summary.job_summaries()[job];
        rows.push(Row::Node(Node::Job(job), self.job_line(job, summary)));
        if self.collapsed_jobs.contains(&job) {
            return;
        }
        for (step, data) in summary_job.steps().iter().enumerate() {
            rows.push(Row::Node(
                Node::Step(job, step),
                self.step_line(job, step, summary),
            ));
            if !self.collapsed_outputs.contains(&(job, step)) {
                self.append_step_output(rows, data, width);
            }
        }
    }

    fn workflow_line(&self, summary: &RunSummaryResponse) -> Line<'static> {
        let mark = fold_mark(self.workflow_expanded, true);
        let label = format!("Workflow: {}", summary.name());
        header_line(0, mark, label, summary.success())
    }

    fn job_line(&self, job: usize, summary: &RunSummaryResponse) -> Line<'static> {
        let summary_job = &summary.job_summaries()[job];
        let expanded = !self.collapsed_jobs.contains(&job);
        let mark = fold_mark(expanded, true);
        let label = format!("Job: {}", job_display(summary, job));
        header_line(1, mark, label, summary_job.success())
    }

    fn step_line(&self, job: usize, step: usize, summary: &RunSummaryResponse) -> Line<'static> {
        let data = &summary.job_summaries()[job].steps()[step];
        let expanded = !self.collapsed_outputs.contains(&(job, step));
        let has_output = !data.stdout().is_empty() || !data.stderr().is_empty();
        let mark = fold_mark(expanded, has_output);
        let label = format!("Step: {}", data.name());
        let mut line = if data.is_skipped() {
            skipped_header_line(2, mark, label)
        } else {
            header_line(2, mark, label, step_success(data))
        };
        push_exit_code(&mut line, data);
        line
    }

    fn append_step_output(&self, rows: &mut Vec<Row>, data: &StepSummaryResponse, width: u16) {
        append_stream(rows, "stdout", data.stdout(), Theme::muted_style(), width);
        append_stream(rows, "stderr", data.stderr(), stderr_style(data), width);
    }
}

impl Default for RunDetailsView {
    fn default() -> Self {
        Self::new()
    }
}

fn row_node(row: &Row) -> Option<Node> {
    match row {
        Row::Node(node, _) => Some(*node),
        Row::Output(_) => None,
    }
}

fn row_item(row: Row) -> ListItem<'static> {
    match row {
        Row::Node(_, line) | Row::Output(line) => ListItem::new(line),
    }
}

fn list_offset(selected: usize, row_count: usize, height: u16) -> usize {
    let viewport = height as usize;
    let target = viewport * 70 / 100;
    let max_offset = row_count.saturating_sub(viewport);
    selected.saturating_sub(target).min(max_offset)
}

fn job_display(summary: &RunSummaryResponse, job: usize) -> String {
    let summary_job = &summary.job_summaries()[job];
    summary_job
        .name()
        .unwrap_or(summary_job.job_id())
        .to_string()
}

fn push_exit_code(line: &mut Line<'static>, step: &StepSummaryResponse) {
    let Some(code) = step.exit_code() else {
        return;
    };
    let style = if code == 0 {
        Theme::body_style()
    } else {
        Theme::signal_style()
    };
    line.spans
        .push(Span::styled(format!("  Exit code: {code}"), style));
}

fn append_stream(rows: &mut Vec<Row>, label: &str, text: &str, style: Style, width: u16) {
    if text.is_empty() {
        return;
    }
    let content_width = output_width(width, label);
    for (index, wrapped) in wrap_lines(text, content_width).into_iter().enumerate() {
        let prefix = if index == 0 {
            format!("{}{}: ", indent(3), label)
        } else {
            indent(3)
        };
        rows.push(Row::Output(Line::from(Span::styled(
            format!("{prefix}{wrapped}"),
            style,
        ))));
    }
}

fn header_line(depth: usize, mark: &str, label: String, success: bool) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{}{}", indent(depth), mark), Theme::muted_style()),
        Span::styled(label, Theme::body_style()),
        Span::raw(" "),
        status_span(success),
    ])
}

fn status_span(success: bool) -> Span<'static> {
    let label = if success {
        format!("[{SUCCESS_LABEL}]")
    } else {
        format!("[{FAILURE_LABEL}]")
    };
    let style = if success {
        Theme::success_style()
    } else {
        Theme::critical_style()
    };
    Span::styled(label, style)
}

fn skipped_header_line(depth: usize, mark: &str, label: String) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{}{}", indent(depth), mark), Theme::muted_style()),
        Span::styled(label, Theme::body_style()),
        Span::raw(" "),
        Span::styled("[SKIPPED]", Theme::signal_style()),
    ])
}

fn fold_mark(expanded: bool, has_children: bool) -> &'static str {
    if !has_children {
        return LEAF_MARK;
    }
    if expanded {
        EXPANDED_MARK
    } else {
        COLLAPSED_MARK
    }
}

fn step_success(step: &StepSummaryResponse) -> bool {
    step.exit_code().is_none_or(|code| code == 0)
}
fn stderr_style(step: &StepSummaryResponse) -> Style {
    if step_success(step) {
        Theme::muted_style()
    } else {
        Theme::critical_style()
    }
}

fn toggle_membership<T: Eq + std::hash::Hash>(set: &mut HashSet<T>, key: T) {
    if !set.remove(&key) {
        set.insert(key);
    }
}

fn indent(depth: usize) -> String {
    "  ".repeat(depth)
}

fn output_width(width: u16, label: &str) -> usize {
    let reserved = indent(3).len() + label.len() + 2;
    (width as usize).saturating_sub(reserved).max(1)
}

fn wrap_lines(text: &str, width: usize) -> Vec<String> {
    let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
    let mut lines = Vec::new();
    for raw in normalized.split('\n') {
        chunk_line(raw, width, &mut lines);
    }
    lines
}

fn chunk_line(raw: &str, width: usize, lines: &mut Vec<String>) {
    let cleaned: String = raw
        .chars()
        .map(|character| {
            if character.is_control() {
                ' '
            } else {
                character
            }
        })
        .collect();
    if cleaned.is_empty() {
        lines.push(String::new());
        return;
    }
    let characters: Vec<char> = cleaned.chars().collect();
    for chunk in characters.chunks(width) {
        lines.push(chunk.iter().collect());
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::{
        application::dtos::responses::{
            JobSummaryResponse, StepSummaryDetails, StepSummaryResponseInput,
        },
        domain::value_objects::StepType,
    };

    fn summary_with_output(output: &str) -> RunSummaryResponse {
        let step = StepSummaryResponse::new(StepSummaryResponseInput::new(
            "compile",
            StepType::Run,
            StepSummaryDetails::new(Some(0), false, Duration::ZERO, output, ""),
        ));
        let job = JobSummaryResponse::new("build", Some("Build".to_string()), vec![step], true);
        RunSummaryResponse::new("CI", vec![job], true, Duration::ZERO)
    }
    fn summary_with_stderr(exit_code: i64, stderr: &str) -> RunSummaryResponse {
        let step = StepSummaryResponse::new(StepSummaryResponseInput::new(
            "compile",
            StepType::Run,
            StepSummaryDetails::new(Some(exit_code), false, Duration::ZERO, "", stderr),
        ));
        let job = JobSummaryResponse::new(
            "build",
            Some("Build".to_string()),
            vec![step],
            exit_code == 0,
        );
        RunSummaryResponse::new("CI", vec![job], exit_code == 0, Duration::ZERO)
    }

    #[test]
    fn successful_stderr_is_rendered_as_muted_diagnostics() {
        let summary = summary_with_stderr(0, "Downloading crates");
        let view = RunDetailsView::new();

        let rows = view.rows(&summary, 80);
        let stderr = rows
            .iter()
            .find_map(|row| match row {
                Row::Output(line) if row_text(row).contains("stderr:") => Some(line),
                _ => None,
            })
            .expect("stderr row");

        assert_eq!(stderr.spans[0].style, Theme::muted_style());
    }

    #[test]
    fn failed_stderr_is_rendered_as_critical_diagnostics() {
        let summary = summary_with_stderr(1, "compiler failed");
        let view = RunDetailsView::new();

        let rows = view.rows(&summary, 80);
        let stderr = rows
            .iter()
            .find_map(|row| match row {
                Row::Output(line) if row_text(row).contains("stderr:") => Some(line),
                _ => None,
            })
            .expect("stderr row");

        assert_eq!(stderr.spans[0].style, Theme::critical_style());
    }

    fn row_text(row: &Row) -> String {
        let line = match row {
            Row::Node(_, line) | Row::Output(line) => line,
        };
        line.spans
            .iter()
            .map(|span| span.content.as_ref())
            .collect()
    }

    #[test]
    fn wrapped_output_has_safe_line_boundaries_and_width() {
        let summary = summary_with_output("first\rsecond\nthird line");
        let mut view = RunDetailsView::new();
        view.collapsed_outputs.clear();
        let rows = view.rows(&summary, 20);
        let text: Vec<String> = rows
            .iter()
            .filter_map(|row| match row {
                Row::Output(_) => Some(row_text(row)),
                Row::Node(_, _) => None,
            })
            .collect();

        assert!(text.iter().any(|line| line.contains("stdout: first")));
        assert!(text.iter().any(|line| line.contains("second")));
        assert!(text.iter().all(|line| line.chars().count() <= 20));
        assert!(text.iter().all(|line| !line.chars().any(char::is_control)));
    }

    #[test]
    fn wrap_lines_treats_carriage_returns_as_line_boundaries() {
        let lines = wrap_lines("first\rsecond\nthird", 40);

        assert_eq!(lines, vec!["first", "second", "third"]);
    }

    #[test]
    fn list_offset_keeps_selection_near_seventy_percent_until_content_ends() {
        assert_eq!(list_offset(14, 100, 20), 0);
        assert_eq!(list_offset(15, 100, 20), 1);
        assert_eq!(list_offset(99, 100, 20), 80);
    }

    #[test]
    fn cursor_stops_at_last_rendered_row() {
        let summary = summary_with_output("first\nsecond\nthird");
        let mut view = RunDetailsView::new();
        view.last_width.set(20);
        view.refresh_row_count(&summary);

        for _ in 0..100 {
            view.move_down(&summary);
        }

        assert_eq!(view.cursor, view.last_row_count.get() - 1);
    }
}
