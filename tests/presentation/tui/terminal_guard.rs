#[cfg(test)]
mod tests {
    use ephact::presentation::tui::terminal_guard::TerminalGuard;

    #[test]
    fn terminal_guard_has_no_runtime_state() {
        assert_eq!(std::mem::size_of::<TerminalGuard>(), 0);
    }
}
