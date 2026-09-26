#[cfg(test)]
mod tests {
    use ephact::presentation::tui::event_reader::EventReader;

    #[test]
    fn event_reader_is_stateless() {
        assert_eq!(std::mem::size_of::<EventReader>(), 0);
    }
}
